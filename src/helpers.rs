use std::path::Path;

// Helper function to check if a file exists
pub fn file_exists(filename: &str) -> bool {
    Path::new(filename).exists()
}

// Helper function that returns a bool if user types y/yes/Y
pub fn confirm(question: &str) -> bool {
    let mut input = String::new();

    println!("{}", question);
    std::io::stdin().read_line(&mut input).unwrap();

    input.trim() == "y" || input.trim() == "yes"
}
