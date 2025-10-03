package main

import (
	"fmt"
	"time"
)

func slowGreet(phrase string) {
    time.Sleep(2 * time.Second)
    fmt.Println("Hello", phrase)
}

func main() {
    // The `go` keyword tells Go to start a goroutine,
    // which allows a function to run concurrently.
    // In simple terms,
    // this means that the function does not block the execution of `main`.
    // So the next lines of code do not have to wait for the greet functions
    // to finish.
    go slowGreet("How ... are ... you ...?")

    println("Go is awesome!")
}

// When you run this code. The only thing that will be printed at the end is
// "Go is awesome".
// This is because when `func main` ends, 
// it does not wait for any of the functions running in goroutines to finish
// their tasks.
// This is not the fault of `func main()`.
// In this example there is no communication between the functions running in
// the goroutines and `func main()`.
