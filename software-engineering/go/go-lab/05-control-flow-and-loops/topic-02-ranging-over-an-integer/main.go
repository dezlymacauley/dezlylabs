/*
    ABOUT: The range over syntax is a newer way to write a `for loop`

    And it is also more idiomatic Go. (The recommended way of doing things)

    With this syntax you don't have to initialize i to 0,
    and you don't have to increment i (i++).

    Go will handle this

*/

package main
import "fmt"

func main() {
	total := 0
    // This will go through the numbers 0 to 2 (excluding 2)
	for i := range 2 {
        total = total + i
        fmt.Println("total is", total)
        // The short hand is: total += i
	}
    // 1
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
