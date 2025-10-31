use std::path::PathBuf;

fn main() {
    //_________________________________________________________________________
    // SECTION: Method 1

    // In this example you use `PathBuf` and not `&PathBuf`
    // because this is not a read-only path.
    // I want this path to be mutable so that I can modify it.
    let mut file_path: PathBuf = PathBuf::from(r"/home/dezlymacauley/");

    file_path.push(r"my-rust-project");
    file_path.push(r"src");
    file_path.push(r"main");
    file_path.set_extension("rs");

    println!("file_path: {}", file_path.display());
    // file_path: /home/dezlymacauley/my-rust-project/src/main.rs

    //_________________________________________________________________________

}
