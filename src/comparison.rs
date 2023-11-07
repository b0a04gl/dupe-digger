

use std::path::PathBuf;
use std::sync::mpsc::Sender;
use std::collections::hash_map::Entry;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

pub fn process_files_for_comparison(
    tx: mpsc::Sender<(u64, Vec<PathBuf>)>,
    results: Arc<Mutex<Vec<(u64, PathBuf)>>>,
) {
    
    let (result_tx, result_rx) = mpsc::channel();

    let mut handles = Vec::new();

    for (fsize, path) in results.lock().unwrap().iter().cloned() {
        let result_tx = result_tx.clone();

        handles.push(thread::spawn(move || {
            let mut duplicates = Vec::new();

            for (size, path) in results.lock().unwrap().iter().cloned() {
                if fsize == size && path != path {
                    duplicates.push(path);
                }
            }

            if !duplicates.is_empty() {
                result_tx.send((fsize, duplicates)).unwrap();
            }
        }));
    }

    
    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    
    let mut grouped_results: HashMap<u64, Vec<PathBuf>> = HashMap::new();
    for (size, paths) in result_rx.iter() {
        if let Entry::Occupied(entry) = grouped_results.entry(size) {
            entry.into_mut().extend(paths);
        } else {
            grouped_results.insert(size, paths);
        }
    }

    for (size, paths) in grouped_results {
        tx.send((size, paths)).unwrap();
    }
}
