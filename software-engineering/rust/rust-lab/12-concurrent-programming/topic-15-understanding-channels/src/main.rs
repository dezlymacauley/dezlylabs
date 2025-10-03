/*
    Imagine you have two workers:
    
    Worker 1 (a separate thread) is packaging boxes in a warehouse
    Worker 2 (main thread) is receiving those boxes at the delivery dock

    A channel is like a conveyor belt between them.

    Worker 1 puts boxes on the belt, Worker 2 picks them up.

*/

// 
use std::thread;
use std::sync::mpsc;

fn main() {
    println!("99-Hello, world!");
}
