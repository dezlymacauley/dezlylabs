// I added the `Error` data type from the io module
// `Error` is one of the possible return types of `read_line()`
// This is there to account for the possibility that the `read_line()` method
// could fail to handle the user input.
use std::io::{stdin, Error, Stdin};

fn main() {
    let mut user_input = String::new();

    println!("Enter a number:");

    let instance_of_stdin: Stdin = stdin();

    // Readline accepts a mutable reference to a String variable type. `&mut`
    // And that String variable type must be declared as mutable as well.
    // E.g. let mut user_input = String::new();
    //
    // In simple terms, read_line(&mut user_input means:
    // 1. I do NOT want to take ownership of the variable called `user_input`
    // because other parts of the program may need to use this variable later.
    //
    // So I'm going to temporarily ask for access to the 
    // variable `user_input` or more specifically, the memory location where
    // it is stored. So `&user_input`
    //
    // 2. However `&user_input` means that I can only read the value,
    // which is not enough because I want to be able to change the value of
    // `user_input` by replacing the empty string that is there,
    // with whatever line of text the user types into the command line.
    //
    // So I need a mutable reference to user_input. Which is `&mut`
    // And this is valid but the variable `user_input` was declared 
    // as mutable.
    let read_line_result: Result<usize, Error> =
        instance_of_stdin.read_line(&mut user_input);

    // Readline returns a result enum:
    // 1. If it successfully reads the user input, and updates `user_input`.
    // Then it returns the number of bytes read. Ok(bytes_read)
    // 2. If there was an error then it returns the Error type. Err(error)
    //
    // Please note that you can name `bytes_read` 
    // and `error` whatever you want.

    //_________________________________________________________________________
    // SECTION: Graceful Error handling of read_line()

    match read_line_result {
        Ok(bytes_read) => {
            println!("Successfully read {} bytes.", bytes_read);
        }
        Err(error) => {
            eprintln!("Failed to read input: {}", error);
        }
    }

    //_________________________________________________________________________
    
    println!("You entered the number: {user_input}");

}
