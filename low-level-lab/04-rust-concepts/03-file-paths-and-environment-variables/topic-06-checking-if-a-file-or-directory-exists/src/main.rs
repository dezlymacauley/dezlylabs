use std::path::Path;

fn main() {
    //_________________________________________________________________________
    // SECTION: Method 1

    let batman_file: &Path = Path::new(r"/home/dezlymacauley/workspace/batman.txt");
    let is_a_dir: bool = batman_file.is_dir();
    let is_a_file: bool = batman_file.is_file();

    println!("Does workspace_dir exist? {is_a_dir}");
    println!("Does batman_file exist? {is_a_file}");
    // Does workspace_dir exist? true
    // Does batman_file exist? false

    //_________________________________________________________________________
}
