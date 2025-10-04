package main

import (
	"fmt"
	"sync"
)

var msg string

var wg sync.WaitGroup

func updateMessage(s string) {
    // NOTE: The `defer` keyword.

    // `defer` means run this function call at the very end of 
    // the current function, just before it returns."
    // So here, wg.Done() will always be called when updateMessage() 
    // finishes, no matter how it ends.
    defer wg.Done()
	msg = s
}

func printMessage() {
	fmt.Println(msg)
}

func main() {
	msg = "Hello, world!"

    wg.Add(1)
	go updateMessage("Hello, universe!")
    wg.Wait()
	printMessage()

    wg.Add(1)
	go updateMessage("Hello, cosmos!")
    wg.Wait()
	printMessage()

    wg.Add(1)
	go updateMessage("Hello, world!")
    wg.Wait()
	printMessage()
}
