use std::path::Path;

// Helper function to check if a file exists
pub fn file_exists(filename: &str) -> bool {
    Path::new(filename).exists()
}
