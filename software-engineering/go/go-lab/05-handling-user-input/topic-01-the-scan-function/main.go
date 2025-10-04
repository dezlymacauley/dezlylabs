package main

import (
	"fmt"
)

func getUserInput(promptText string) string {

    // This variable will be used to store the unprocessed string 
    // that the user types into the terminal.
    var unprocessed_input string = ""
    
    // `fmt.Print` is used for prompts because unlike `fmt.Println`,
    // it does not add a newline character.
    // So you can do things like:
    // "What is your name: Then the user types in their name here"
	fmt.Print(promptText)
    
    // This will read whatever the user types into the terminal and then
    // store it in the variable `unprocessed_input`

    // Scan needs to modify the variable `unprocessed_input`, 
    // so this is why you pass in `&unprocessed_input`.
    // The `&` sign means 
    // `the memory address of the variable unprocessed_input`
    // Also note that in Go the contents, 
    // of a variable declared as a string is immutable be default.
    // So for this update to happen, the `Scan` function actually 
    // creates a new string from the what the user typed in the terminal,
    // and then assigns that string to the variable unprocessed_input.

    // If you just used `fmt.Scan(unprocessed_input)`, you would be saving
    // the user input from the terminal into a copy of 
    // the variable `unprocessed_input`, and that copy would be discarded
    // as soon as `Scan` is done. 
    // This would result in a bug because the original variable 
    // `unprocessed_input` would not be updated.
	fmt.Scan(&unprocessed_input)

    // Then the function will return the unprocessed_input to the caller  
	return unprocessed_input
}

func main() {
	firstName := getUserInput("Enter your first name: ")
	fmt.Printf("Greetings %s\n", firstName)
}
