use std::thread;

fn main() {
    println!("Message A");

     let handle = thread::spawn(|| {
        println!("Message B");
    });

    println!("Message C");

    // This tells the current thread `fn main()`,
    // not to exit until the the thread that prints "Message B" is done.
    handle.join().unwrap();
}
