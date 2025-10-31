/*

ABOUT: Relative file paths

This is about selecting files relative to the root of the project.
In simple terms, selecting files inside the Rust project.

*/

use std::path::{Path, PathBuf};

fn main() {
    let project_root: &'static str = env!("CARGO_MANIFEST_DIR");

    // `r` before the str value tells Rust to treat this as a raw sting.
    // In simple terms, you want Rust to interpret the string exactly as
    // you entered it, and to not perform any special actions 
    // when it encounters and escape character like `\n`, `\t`, or `\\`

    // 
    let csv_data_dir: PathBuf = Path::new(project_root).join(r"csv-data");

    // `.display()` will temporarily add the `display` trait to `csv_data_dir`
    // which is of the type `PathBuf`, so that it can be printed 
    
    // in a human-readable format. 
   
    // `.display()` wraps csv_data_dir in 
    // a Display struct (std::path::Display) so that is can be printed 
    // in a human-readable format.
    println!("csv_data_dir: {}", csv_data_dir.display());
    // csv_data_dir: /home/dezlymacauley/workspace/github-repos/dezlylabs/rust-lab/03-file-paths/topic-02-relative-file-paths/csv-data
}
