use std::thread;

fn main() {
    let mut thread_vec = vec![];

    for i in 1..=10 {
        thread_vec.push(

            thread::spawn(
                move|| {
                    println!("This is thread {i}");
                }
            )

        );
    }

    for i in thread_vec {
        i.join().unwrap();
    }

    // This is thread 3
    // This is thread 2
    // This is thread 4
    // This is thread 5
    // This is thread 1
    // This is thread 6
    // This is thread 7
    // This is thread 8
    // This is thread 9
    // This is thread 10

}
