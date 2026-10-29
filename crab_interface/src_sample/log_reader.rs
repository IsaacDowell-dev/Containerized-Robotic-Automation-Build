use std::env;
use std::fs;
use std::path::Path;

pub fn read_contents()
{
    println!("Working Directory: {:?}", env::current_dir());

    let file_path = Path::new("test.txt");

    let contents = fs::read_to_string(file_path)
        .expect("Failed to read file");

    println!("Contents: {contents}")
}