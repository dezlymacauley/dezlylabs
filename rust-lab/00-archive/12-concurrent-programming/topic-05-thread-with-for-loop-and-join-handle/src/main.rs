use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

fn main() {
    let handle: JoinHandle<()> = thread::spawn(|| {
        for i in 1..=11 {
            println!("counter in thread is {i}");
            thread::sleep(Duration::from_secs(2));
        }
    });

    for i in 1..=11 {
        println!("counter in main is {i}");
        thread::sleep(Duration::from_secs(1));
    }

    // The `.join()` method blocks the main thread from exiting 
    // until the spawned thread has completed all of its tasks.

    // So once the main thread gets to `counter in main is 11`
    // the program will wait for the spawned thread to display
    // `counter in thread is 11`, and only then will it exit.

    // `.join()` and `sleep()` are blocking functions.

    // TODO: It was here
    
    handle.join().unwrap();
    println!("Spawned thread has finished");

    // NOTE: Both threads complete their tasks and then the program ends.

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
    // counter in thread is 7
    // counter in thread is 8
    // counter in thread is 9
    // counter in thread is 10
    // counter in thread is 11
    // Spawned thread has finished
}
