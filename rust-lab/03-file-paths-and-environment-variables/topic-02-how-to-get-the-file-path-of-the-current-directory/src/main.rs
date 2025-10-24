use std::{env::current_dir, path::PathBuf};

fn main() {
    let current_dir: PathBuf = current_dir().unwrap();
    println!("current_dir: {}", current_dir.display());
    // current_dir: /home/dezlymacauley/workspace/github-repos/dezlylabs/rust-lab/003-file-paths/topic-007-how-to-get-the-file-path-of-the-current-directory/csv-files

    // NOTE: For this to work:
    // cd csv-files
    // Then run `cargo rq`
}
