mod hashing;
use hashing::process_files_for_hashing;


#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::thread;

    
    fn fake_hash_file_and_send(
        tx: &mpsc::Sender<(u64, PathBuf, Vec<u8>)>,
        path: PathBuf,
    ) {
        let fake_hash: Vec<u8> = vec![1, 2, 3];
        let fake_size: u64 = 12345;
        tx.send((fake_size, path, fake_hash)).unwrap();
    }

    #[test]
    fn test_process_files_for_hashing() {
        
        let (tx, rx) = mpsc::channel();

        
        let minsize: u64 = 0;
        let maxsize: u64 = u64::MAX;
        let verbose: bool = false;
        let roots: Vec<PathBuf> = vec![PathBuf::from("/test_root")];

        
        process_files_for_hashing(&tx, minsize, maxsize, verbose, roots);

        
        fake_hash_file_and_send(&tx, PathBuf::from("test_file.txt"));

        
        let (fsize, path, hash) = rx.recv().unwrap();

        
        assert_eq!(fsize, 12345);
        assert_eq!(path, PathBuf::from("test_file.txt"));
        assert_eq!(hash, vec![1, 2, 3]);
    }
}
