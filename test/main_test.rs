#[cfg(test)]
mod main_tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::TempDir; 

    #[test]
    fn test_hash_file() {
        let temp_dir = TempDir::new().expect("Failed to create a temporary directory");
        let test_file_path = temp_dir.path().join("test.txt");

        
        let content = b"Hello, world!";
        let mut test_file = File::create(&test_file_path).expect("Failed to create a test file");
        test_file.write_all(content).expect("Failed to write to the test file");

        let hash = hash_file(&test_file_path).expect("Failed to hash the file");
        let expected_hash = hex_digest(Algorithm::SHA256, content);

        assert_eq!(hash, expected_hash);
    }

}
