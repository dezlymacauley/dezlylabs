/*
    ABOUT: Channel vs Mutex

    Channel:
    A channel is used when you want to pass data or messages between threads,
    especially when the data is produced in one thread and consumed in
    another thread.
    
    This is great for tasks that can be neatly divided 
    and handled by different threads.

    Mutex (Short for mutal exclusion):
    This is for situations where mutliple threads need to access 
    and potentially modify shared data.

    NOTE: In this example, there is no concurrency so a Mutex is overkill

    The reason I'm using this example is to keep things simple and focused
    on how to use `Mutex` pattern in Rust.

*/

// First you need to bring `Mutex` into scope, from the `sync` module,
// which is part of the Rust standard library.
// The sync module contains useful tools for keeping data in sync during 
// concurrent programming.
use std::sync::{Mutex, MutexGuard};

fn check_balance(account: &Mutex<i32>) {
    // When a part of the program wants to read or modify the value of account,
    // that part of the program must first request exclusive access to the 
    // value of account using `.lock()`
    // `.lock()` has the potential to fail so you must handle the error.
    // In this case I'm just using `.unwrap()` which is not graceful.
    // If this succeeds the lock (exclusive access) will be granted to the 
    // variable `balance`.

    // `MutexGuard` is the smart pointer returned by `lock()`.
    // It provides access to the data inside the Mutex and
    // automatically releases the lock when it goes out of scope.
    let balance: MutexGuard<'_, i32> = account.lock().unwrap();

    // Now that the lock has been acquired by the variable `balance`
    // we simply dereference balance to get its value.
    println!("Current balance: ${}", *balance);
    // After this function call ends and `balance` goes out of scope,
    // the lock will be automatically released.
}

fn deposit_amount(account: &Mutex<i32>, amount: i32) {

    // In this case the `mut` keyword is added because I want to request
    // a lock (exclusive access to the Mutex account) and I also want to
    // be able to modify the value of the i32.
    // The Mutex guarantees that while this function is modifying the account
    // balance, no other part of the program will be able to read the account
    // balance (prevents an incorrect reading), 
    // or modifiy it (prevents a data race).
    let mut balance = account.lock().unwrap();
    *balance += amount;
    println!("Deposited ${}. New balance: ${}", amount, *balance);
}

fn withdraw_amount(account: &Mutex<i32>, amount: i32) {
    let mut balance = account.lock().unwrap();
    *balance -= amount;
    println!("Withdrew ${}. New balance: ${}", amount, *balance);
}

fn main() {
    // Create a bank account with $850, protected by a Mutex

    // This is a bank account with $850
    // This is fine for synchronous programming, 
    // but you could run into issues with concurrent programming if different
    // parts of the program try to modify this value at the same time.
    // let account: i32 = 100;

    // This is where `Mutex` (short for mutual exclusion lock) comes in.
    // It guarantees that only one part of the program can access the account
    // at a time - whether reading or modifying it.
    // In simple terms, there can only be one reader OR one writer at a time.

    // NOTE: 
    // 1. When you use the Mutex wrapper on a variable, 
    // you ALWAYS declare it as immutable.
    // ---------------------------------------------------
    // 2. The Mutex itself handles the mutability of the data inside (the i32)
    // through its locking mechanism.
    // ---------------------------------------------------
    // 3. When a part of the program needs 
    // to access the internal value (the i32), then that part of the 
    // program will have to make it clear whether it wants 
    // immutable (read-only) access to the i32 value or wants mutable access
    // using the `mut` keyword.
    // ----------------------------------------------------
    // 4. In simple terms, you don't worry about the mutability 
    // at the declaration level like a regular variable.
    // Mutabilty will be set on a per-data-request level.
    let account: Mutex<i32> = Mutex::new(850);

    check_balance(&account);
    // Current balance: $850

    deposit_amount(&account, 50);
    // Deposited $50. New balance: $900
    
    check_balance(&account);
    // Current balance: $900

    withdraw_amount(&account, 200);
    // Withdrew $200. New balance: $700
    
    check_balance(&account);
    // Current balance: $700

}
