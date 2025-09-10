/*

ABOUT: Thread

A thread is a block of code that runs separately from the main thread
(aka fn main).

The easiest way to think of a thread,
is to view it as a side program that runs alongside the main thread,
without blocking its flow.

*/

// This will bring the `thread` functionality from the Rust standard library
// into scope.
use std::thread;

fn main() {
    println!("Message A");

    // `thread::spawn` creates and starts a new thread.
    // The spawn function returns a join handle.
    thread::spawn(|| {
        println!("Message B");
    });

    println!("Message C");

    // Message B will not be printed
}
