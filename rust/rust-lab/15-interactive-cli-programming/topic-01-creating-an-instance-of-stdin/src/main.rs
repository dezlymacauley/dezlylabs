// `io` stands for `input output`. 
// It is a module from the Rust standard library.
//
// A module is just a collection of Rust code that contains functions, 
// data structures, and variable types that can be imported into your program. 
//
// To be efficient, I am using the named import syntax to specify the exact
// tools from the `io` module that I will be using.
//
// I will be importing two things:
// 1. The `stdin` function
// 2. The `Stdin` variable type. 
// This is the variable type that is returned by the `stdin` function.

use std::io::{stdin, Stdin};

// This is just to silence the warnings in this file
#[allow(unused_variables, unused_mut)]
fn main() {

    // The variable `user_input` will be used to collect the user input.
    // It is currently set to an empty string that can be replaced with the
    // actual user input that will be recieved later.
    let mut user_input = String::new();

    println!("Enter a number:");

    //_________________________________________________________________________
    
    // io::stdin() returns an instance of a `Stdin` struct.
    // This struct contains useful methods for getting user input.
    let instance_of_stdin: Stdin = stdin();

    //_________________________________________________________________________

}
