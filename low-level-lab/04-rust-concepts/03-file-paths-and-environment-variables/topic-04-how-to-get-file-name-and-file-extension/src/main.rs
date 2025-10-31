/*

ABOUT: OsStr

This is used to represent strings exactly they are stored 
by the operating system. So it is a platform-specific variable.

Please note the OS strings may contain characters that are not UTF-8.

*/
// It represents strings as stored by the OS, not necessarily UTF-8
use std::{ffi::OsStr, path::Path};

fn main() {
    // The &Path is used for immutable read-only paths.
    let csv_file_path: &Path = Path::new(r"/home/dezlymacauley/csv-data/my_csv_data.csv");

    // This could fail if the path has no file name.
    let file_name: &OsStr = csv_file_path.file_stem().unwrap();

    // This could fail if the path has no file extension.
    let file_extension: &OsStr = csv_file_path.extension().unwrap();
    
    println!("csv_file_path: {}", csv_file_path.display());
    println!("file_name: {}", file_name.display());
    println!("file_extension: {}", file_extension.display());

    // csv_file_path: /home/dezlymacauley/csv-data/my_csv_data.csv
    // file_name: my_csv_data
    // file_extension: csv
}
