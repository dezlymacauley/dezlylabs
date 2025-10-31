package main

import (
	"fmt"
)

func viewAge(age uint8) {
    fmt.Printf("userAge is: %v\n", age)

    // `p` is the format specificier for printing a memory address
    // The `p` is for pointer.
    fmt.Printf("The value of age is stored at: %p\n", &age)
}

func main() {
	var userAge uint8 = 35
    fmt.Printf("userAge is: %v\n", userAge)
    // userAge is: 35

    fmt.Printf("The value of userAge is stored at: %p\n", &userAge)
    // The value of userAge is stored at: 0xc00009a008

    fmt.Println()

    // NOTE: This function accepts a `uint8` so this is pass by value.
    // A copy of the variable `userAge` is passed into the function.

    // In this situation, `pass by value` is fine because I simply want to
    // read the value of the variable.

	viewAge(userAge)
    // userAge is 35
    // The value of age is stored at: 0xc00009a009
}
