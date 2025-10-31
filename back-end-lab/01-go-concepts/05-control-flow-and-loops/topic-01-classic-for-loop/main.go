package main

import "fmt"

func main() {
	total := 0

    // This will go through the numbers 0 to 2 (excluding 2)
	for i := 0; i < 2; i++ {
        total = total + i
        fmt.Println("total is", total)
	}

    // First loop:  
    // Start of loop => total = 0 and  i = 0
    // End of loop => Total = 0 + 0 = 0 
    // i increased to 1

    // Second loop:  
    // Start of loop => total = 0 and  i = 1
    // End of loop => Total = 0 + 1 = 1
    // i increased to 2
    // The for loop will not run again because i is no longer less than 2

}
