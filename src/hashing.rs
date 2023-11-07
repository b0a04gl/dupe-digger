

use std::path::PathBuf;
use std::sync::mpsc::Sender;

pub fn process_files_for_hashing(
    tx: &mpsc::Sender<(u64, PathBuf, Vec<u8>)>,
    minsize: u64,
    maxsize: u64,
    verbose: bool,
    roots: Vec<PathBuf>,
) {
    
    let results = Arc::new(Mutex::new(Vec::new()));

    
    let hash_and_send = |path: PathBuf| {
        if verbose {
            eprintln!("Hashing {}...", path.display());
        }
        if let Ok(hash) = hash_file_inner(&path) {
            if let Ok(metadata) = path.metadata() {
                let fsize = metadata.len();
                tx.send((fsize, path, hash)).unwrap();
            } else {
                eprintln!("Error getting metadata for {}", path.display());
            }
        } else {
            eprintln!("Error hashing file: {}", path.display());
        }
    };

    for root in &roots {
        let mut handles = Vec::new();

        for entry in WalkDir::new(root).follow_links(false) {
            if let Ok(entry) = entry {
                if entry.file_type().is_file() {
                    let path = entry.path().to_path_buf();
                    if let Ok(metadata) = path.metadata() {
                        let fsize = metadata.len();
                        if fsize >= minsize && fsize <= maxsize {
                            let results = Arc::clone(&results);
                            let tx = tx.clone();

                            handles.push(thread::spawn(move || {
                                hash_and_send(path);
                            }));
                        }
                    }
                }
            }
        }

        
        for handle in handles {
            handle.join().expect("Thread panicked");
        }
    }
}
