/*
   ABOUT: Go does not have the `while` keyword like other languages

   In Go you just use a simple for loop
*/

package main

import "fmt"

func main() {

	numberOfApples := 0

	for numberOfApples < 10 {
		numberOfApples = numberOfApples + 2
		fmt.Printf("Number of apples is %d\n", numberOfApples)
	}
    // Number of apples is 2
    // Number of apples is 4
    // Number of apples is 6
    // Number of apples is 8
    // Number of apples is 10

}
