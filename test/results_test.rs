

mod results;
use results::collect_and_print_results;

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::path::PathBuf;
    use std::thread;

    #[test]
    fn test_collect_and_print_results_singleline() {
        
        let (tx, rx) = mpsc::channel();

        
        let data: Vec<(u64, Vec<PathBuf>)> = vec![
            (100, vec![PathBuf::from("file1"), PathBuf::from("file2"), PathBuf::from("file3")])
           
        ];

        
        thread::spawn(move || {
            for item in data {
                tx.send(item).unwrap();
            }
        });

        
        collect_and_print_results(rx, true, false, false);
    }

    #[test]
    fn test_collect_and_print_results_multiline() {
        
        let (tx, rx) = mpsc::channel();

        
        let data: Vec<(u64, Vec<PathBuf>)> = vec![
            (100, vec![PathBuf::from("file1"), PathBuf::from("file2")]),
            (200, vec![PathBuf::from("file3")]),
        ];

        
        thread::spawn(move || {
            for item in data {
                tx.send(item).unwrap();
            }
        });

        
        collect_and_print_results(rx, false, false, false);
    }
}
