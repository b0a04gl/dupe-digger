use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::collections::hash_map::Entry;
use crypto_hash::{Algorithm, hex_digest};
use walkdir::WalkDir;

fn hash_file(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let hash = hex_digest(Algorithm::SHA256, &buffer);
    Ok(hash)
}

fn find_duplicates(directory: &Path) -> io::Result<HashMap<u64, Vec<PathBuf>>> {
    let mut size_to_files: HashMap<u64, Vec<PathBuf>> = HashMap::new();

    for entry in WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
            let entry_path = entry.path().to_path_buf();

            match size_to_files.entry(size) {
                Entry::Vacant(v) => {
                    v.insert(vec![entry_path]);
                }
                Entry::Occupied(mut v) => {
                    v.get_mut().push(entry_path);
                }
            }
        }
    }

    let mut duplicate_files: HashMap<u64, Vec<PathBuf>> = HashMap::new();

    for (size, files) in size_to_files.iter() {
        if files.len() > 1 {
            let mut content_hash_to_files: HashMap<String, Vec<PathBuf>> = HashMap::new();

            for file in files.iter() {
                if let Ok(content_hash) = hash_file(file) {
                    match content_hash_to_files.entry(content_hash) {
                        Entry::Vacant(v) => {
                            v.insert(vec![file.clone()]);
                        }
                        Entry::Occupied(mut v) => {
                            v.get_mut().push(file.clone());
                        }
                    }
                }
            }

            for (_, duplicate_files_list) in content_hash_to_files.iter() {
                if duplicate_files_list.len() > 1 {
                    duplicate_files.insert(*size, duplicate_files_list.clone());
                }
            }
        }
    }

    Ok(duplicate_files)
}

fn main() -> io::Result<()> {
    let directory = std::env::args()
        .nth(1)
        .expect("Please provide a directory path");

    let directory = Path::new(&directory);

    if !directory.is_dir() {
        eprintln!("The provided path is not a directory.");
        std::process::exit(1);
    }

    let duplicate_files = find_duplicates(directory)?;

    for (size, files) in duplicate_files.iter() {
        println!("Size: {} bytes", size);
        for file in files.iter() {
            println!("  - {:?}", file);
        }
    }

    Ok(())
}
