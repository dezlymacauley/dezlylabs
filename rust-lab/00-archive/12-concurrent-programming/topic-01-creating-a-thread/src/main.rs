/*

ABOUT: Thread

Here's a good anology:

_______________________________________________________________________________c

1. A default Rust program:

Think of this as a company that has only one office, 
and one office manager.

The office is called `fn main`, and the manager is `The Rust Compiler`.

You can think of the `The Rust Compiler` as the most import manager
of the company. Aka the CEO of the company.

The Rust Compiler is responsible for the `main` progress of the company.

_______________________________________________________________________________

2. A Thread:

Think of a thread as an office that your program is given
to complete all the tasks that you have told it to perform.

_______________________________________________________________________________

3. A Join Handle:

Think of this as the office manager for a specific office.
This manager opens the office, ensures that work is done,
and then reports back to the CEO (aka the Rust compiler) when work is done.

_______________________________________________________________________________

Just like how a company can have mutiple offices and mutiple managers,
it is also possible for a Rust program to have multiple threads and mutiple
join handles.

_______________________________________________________________________________c

*/

// This line tells the Rust compiler that I intend to use the `thread` module
// from the Rust standard library.
//
// This is a module that contains many useful functions
// for creating your own threads and managing those threads.
use std::thread;

// This is the variable type of the join handle that will be used to manage
// threads.
use std::thread::JoinHandle;

// When run your compiled Rust program,
// the operating system gives your program one thread by default.
//
// The default thread for a Rust program is `fn main()`.
fn main() {
    // This will be printed by the main thread.
    println!("Message A");

    // When you want to create a thread in Rust,
    // the first thing that you have to do is to create a `join handle` aka
    // an office manager.

    // `thread::spawn` creates and starts a new thread.
    // The spawn function returns a join handle.

    // The variable type is JoinHandle<T>
    // T is the variable type that is returned by the closure.
    //
    // Currently I am not returning anything back from the closure,
    // I am simply printing to the terminal so T is ().
    // () is called the unit type. It is used to show that a function
    // does not return anything.
    let join_handle: JoinHandle<()> = thread::spawn(|| {
        println!("Message B");
    });

    // After "Message A" is printed. The program will skip ahead to the next
    // instruction and print "Message C". This is because "Message B" is
    // being processed in a different thread outside the flow of `fn main()`
    println!("Message C");

    // To continue the anology.
    // This is the office manager in action.
    // `.join()` tells the main thread `fn main()`, to not proceed with
    // the program until it gets the work that is being processed
    // by the office manager's office.

    // In simple terms, `fn main()` is told:
    // Don't print `Message D`, until you have received message "Message B"
    // from me. So `.join()` is a blocking method.

    // `.unwrap()` tells the program to crash if immediately if it fails to
    // get a response ("Message B")
    join_handle.join().unwrap();

    println!("Message D");
}

// This is what will be printed.
// Message A
// Message C
// Message B
// Message D
