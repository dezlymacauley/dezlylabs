use std::{thread, time::Duration};

fn main() {
    thread::spawn(|| {
        for i in 1..=11 {
            println!("counter in thread is {i}");
            thread::sleep(Duration::from_secs(2));
        }
    });

    for i in 1..=11 {
        println!("counter in main is {i}");
        thread::sleep(Duration::from_secs(1));
    }

    // counter in main is 1
    // counter in thread is 1
    // counter in main is 2
    // counter in thread is 2
    // counter in main is 3
    // counter in main is 4
    // counter in thread is 3
    // counter in main is 5
    // counter in main is 6
    // counter in thread is 4
    // counter in main is 7
    // counter in main is 8
    // counter in thread is 5
    // counter in main is 9
    // counter in main is 10
    // counter in main is 11
    // counter in thread is 6

    // NOTE: The counter in the the thread never gets to 11
}
