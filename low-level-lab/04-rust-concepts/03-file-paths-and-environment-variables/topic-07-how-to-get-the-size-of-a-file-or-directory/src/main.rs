use std::path::Path;

fn main() {
    let selected_file: &Path = Path::new(r"/home/dezlymacauley/workspace/batman.txt");

    let is_regular_file: bool = selected_file.metadata().unwrap().file_type().is_file();
    let file_size: u64 = selected_file.metadata().unwrap().len();

    println!("is_regular_file: {}", is_regular_file);
    println!("file_size: {} bytes", file_size);
    // is_regular_file: true
    // file_size: 17 bytes
}
