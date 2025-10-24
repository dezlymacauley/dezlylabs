use std::fs::{self, File};

fn main() {
    // To create a single file
    File::create("file_one.txt").unwrap();
    println!("File created successfully!");
    
    // To create a file in a directory
    fs::create_dir_all("directory-a/sub-directory-one").unwrap();
    File::create("directory-a/sub-directory-one/file_two.txt").unwrap();

    println!("Nested file created successfully!");

}
