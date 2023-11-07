use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

mod hashing;
mod comparison;
mod printing;

use hashing::process_files_for_hashing;
use comparison::process_files_for_comparison;
use results::collect_and_print_results;



fn main() {
    

    
    let (hash_tx, hash_rx) = mpsc::channel();

    
    let (compare_tx, compare_rx) = mpsc::channel();

    
    let roots_for_hash = roots.clone();
    let roots_for_compare = roots.clone();

    
    thread::spawn(move || {
        process_files_for_hashing(&hash_tx, minsize, maxsize, verbose, roots_for_hash);
    });

    
}
