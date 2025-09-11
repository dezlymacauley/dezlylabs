use std::thread;
use std::time::Duration;

fn main() {
    println!("Message A");

    thread::spawn(|| {
        println!("Message B");
    });

    println!("Message C");

    // This tells the main thread to pause for 2 seconds before completing
    // the program.
    // This will allow `Message B` to be printed.
    thread::sleep(Duration::from_secs(2));
}
