package main

import (
	"fmt"
)

// You put a `*` before the variable type to show that it is a reference 
// to a variable of the type *uint8
func updateAge(currentAge *uint8, newAge uint8) {

    // `%` is the format specifier for printing out a memory address
    // The `p` if for pointer.
    fmt.Printf("The value of userAge is stored at: %p\n", currentAge)

    // NOTE: To access the value of a pointer, 
    // you need to deference it by putting an asterisk `*` before 
    // the variable name.

    // In this line I am deferencing `currentAge` to access its value,
    // so that I can print it.
    fmt.Printf("userAge is currently: %v\n", *currentAge)

    // In this line I am deferencing `currentAge` to access its value,
    // so that I can update it.
    *currentAge = newAge

    fmt.Printf("userAge has been updated to: %v\n", *currentAge)

}

func main() {
	var userAge uint8 = 35
    fmt.Printf("userAge is: %v\n", userAge)
    // userAge is: 35

    fmt.Printf("The value of userAge is stored at: %p\n", &userAge)
    // The value of userAge is stored at: 0xc0000120c8

    //_________________________________________________________________________
    fmt.Println()

    // NOTE: This function accepts a `*uint8` so this is pass by reference.
    // The memory address of the variable `userAge` is passed into 
    // the function.

    // The function will be able to modify the value userAge,
    // by deferencing the original

	updateAge(&userAge, 40)
    // The value of userAge is stored at: 0xc0000120c8
    // userAge is currently: 35
    // userAge has been updated to: 40

}
