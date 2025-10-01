/*
    ABOUT: Creating Channels

    Do not communicate by sharing memory; instead, 
    share memory by communicating

    Channels simplify concurrent programming by avoiding the complexity of
    shared state.

    Each channel consists of two parts:
    1. Sender (Usually stored in a variable called `tx` which is short for
    transmitter).
    
    Responsible for sending messages. It can be cloned to allow muliple
    producers.
    
    2. Receiver (rx)
    Waits for messages to be sent through the channel.

    A channel is closed when all of the senders and the receivers are no 
    longer available.
*/

use std::sync::mpsc;
use std::thread;

fn main() {
    // Create a channel
    let (tx, rx) = mpsc::channel();

    thread::spawn(
        move|| {
            tx.send("Hello from spawned thread").unwrap();
        }
    );

    // .recv() is a blocking method
    // It will block the main thread until it receives the message from the
    // spawned thread.
    match rx.recv() {
        Ok(message) => println!("received messsage: {message}"),
        Err(error) => println!("An error occured: {error}")
    }

}
