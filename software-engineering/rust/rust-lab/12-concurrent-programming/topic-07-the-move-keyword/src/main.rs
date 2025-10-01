use std::thread;

// NOTE: If the thread captures a primitive type that implements the copy
// trait, then the data is copied into to spawned thread.

// In this case, the variable `message` is of the type `String`,
// which is NOT a primitive type.
// String owns Heap memory so it can't be copied. It must be moved.

fn main() {
    let message: String = String::from("hello from the closure");

    // The spawn thread needs to take ownership of the `message` because
    // it can't garantee that the message will live long enough in the main
    // thread.

    // The move keyword garantees that `message` will not be dropped,
    // while the spawned thread below is using it.
    thread::spawn(
        move|| {
            println!("{message}");
        }
    ).join().unwrap();
    // You can join directly on a spawned thread.
    
}
