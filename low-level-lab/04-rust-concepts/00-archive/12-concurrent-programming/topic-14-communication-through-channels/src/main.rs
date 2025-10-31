/* 
    ABOUT: Channels

    FIFO - First in, First Out
*/

use std::thread;

// `mpsc` stands for multiple producer, single consumer
use std::sync::mpsc;

fn main() {
    // tx stands for transmitter
    // rx stands for receiver
    let (tx, rx) = mpsc::channel();

    // The spawned thread needs to own the transmitter.
    
    let t = thread::spawn(
        move|| {
           let val: String = String::from("some data from sender");
           println!("Value sending from thread");
           tx.send(val).unwrap();
           println!("This may execute after the statement in main");
        }
    );

    // let recieved: String = rx.recv().unwrap();
    // println!("Recieved: {recieved:?}");

    // The main function will be blocked until the thread is recieved.

    // Value sending from thread
    // This may execute after the statement in main
    // Recieved: "some data from sender"

    t.join();
    let mut recieved_status: bool = false;
    while recieved_status != true {
        match rx.try_recv() {
            Ok(recieved_value) => {
                println!("Recieved value is {recieved_value:?}");
                recieved_status = true;
            },
            Err(_) => println!("I am doing some other stuff"),
        }
    }
}
