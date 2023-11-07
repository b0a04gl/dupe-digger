#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_process_files_for_comparison() {
        
        let (tx, rx) = mpsc::channel();

        
        let results_data = vec![(100, PathBuf::from("file1")), (200, PathBuf::from("file2")), (100, PathBuf::from("file3"))];
        let results = Arc::new(Mutex::new(results_data));

        
        process_files_for_comparison(tx.clone(), Arc::clone(&results));

        
        thread::sleep(Duration::from_secs(1));

        
        let mut collected_results = HashSet::new();
        while let Ok(result) = rx.try_recv() {
            collected_results.insert(result);
        }

        
        let expected_results: HashSet<(u64, Vec<PathBuf>)> = vec![(100, vec![PathBuf::from("file2")])].into_iter().collect();
        assert_eq!(collected_results, expected_results);
    }
}
