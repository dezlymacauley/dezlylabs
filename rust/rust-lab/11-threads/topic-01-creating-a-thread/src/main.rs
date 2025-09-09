use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Spawned thread: I'm finishing my job first");
    });

    handle.join().unwrap(); 

    println!("Main thread: Now I run after the spawned thread");
}
