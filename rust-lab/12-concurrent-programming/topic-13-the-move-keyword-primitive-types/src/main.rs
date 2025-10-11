use std::thread::{self, JoinHandle};

fn main() {
    let account_balance: i32 = 850;

    // When you write a closure, 
    // it normally borrows things from the outside scope if it can.
    // Without move: the closure tries to use references (&T) 
    // to your variables.
    // With move: the closure captures values 
    // by value (takes ownership, or copies if the type is Copy).
    let join_handle: JoinHandle<()> = thread::spawn(move || {
        println!("The account balance is {account_balance}");
    });

    join_handle.join().unwrap();

    println!("The account balance is {account_balance}");
}
