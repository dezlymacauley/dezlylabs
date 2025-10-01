use std::thread;
use std::thread::JoinHandle;

fn main() {
    // create a vec to store the joinhandles of the threads we'll create
    let mut handles: Vec<JoinHandle<()>> = vec![];

    for i in 1..11 {
        // The `move` keyword will move the ownership of the variable
        // that the closure captures
        let handle: JoinHandle<()> = thread::spawn(move || {
            println!("counter in thread is {}", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
