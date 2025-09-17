/*

ABOUT: Input sanitation

This is about making sure that the collected user_input is safe before parts
of your program start using it.

This involves the identification and removal of any malicious character
combination that hackers use to:
1. Disrupt the functionality of your program
2. Insert malicious code.
3. Perform unauthorized actions.

This is a preventative technique to prevent common attacks 
like SQL injections and XSS (cross site scripting).

*/

use std::io::{stdin, Error, Stdin};

fn main() {
    let mut user_input = String::new();

    println!("Enter a number:");

    let instance_of_stdin: Stdin = stdin();

    let read_line_result: Result<usize, Error> =
        instance_of_stdin.read_line(&mut user_input);

    match read_line_result {
        Ok(bytes_read) => {
            println!("Successfully read {} bytes.", bytes_read);
        }
        Err(error) => {
            eprintln!("Failed to read input: {}", error);
        }
    }
    //_________________________________________________________________________
    // TODO: Sanitizing user_input


    //_________________________________________________________________________

    // println!("You entered the number: {user_input}");
}
