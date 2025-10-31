/*

ABOUT: "CARGO_MANIFEST_DIR"

This is a special environment variable that is automatically set by `cargo`
during compile time.

It is the absolute path to the directory that contains the `Cargo.toml` file.

Since the `Cargo.toml` file is at the root of the Rust project,
this is an easy way to get the project root.

*/

fn main() {
    // `env!()` is a macro that will get the environment variable
    // "CARGO_MANIFEST_DIR" and replace it with a string literal.

    // A string literal is an immutable value that 
    // is not stored on the Stack or Heap,
    // it is stored in the compiled binary of the program. In otherwords,
    // it is read-only.

    // `&'static str` is Rust's way of ensuring that the string literal
    // `project_root` will be valid for the entire duration of the program.
    // Therefore, it is safe to use.
    
    let project_root: &'static str = env!("CARGO_MANIFEST_DIR");
    println!("project_root: {project_root}");
    // project_root: /home/dezlymacauley/workspace/github-repos/dezlylabs/rust-lab/03-file-paths/topic-01-how-to-get-the-project-root
}
