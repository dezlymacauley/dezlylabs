use std::fs;

fn main() {
    // To create a single directory
    fs::create_dir("directory-a").unwrap();
    println!("Directory created successfully!");

    // To created a nested directory
    fs::create_dir_all("directory-b/sub-directory-one").unwrap();
    println!("Directory created successfully!");
}
