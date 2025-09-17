/*

ABOUT: Error Handling

Certain functions in Rust have the potential to fail, 
simply due to the nature of the action being performed.

In these situtions, the Rust compiler requires you to explicitly tell it how
it should proceed with the rest of the program (recoverable error),
or when it should crash the program 
and not proceeed further (unrecoverable error).

If there is any uncertainity or ambiguity, Rust will simply refuse to compile
your code.

This requirement makes your program more robust by ensuring that 
errors and handled at compile time (i.e. before you application is deployed
to production).

1. Recoverable Errors:
- Handled with the `Result<T, E>` enum

2. Unrecoverable Errors:
- Handled with the `panic!` macro

*/

fn main() {
    println!("Hello, world!");
}
