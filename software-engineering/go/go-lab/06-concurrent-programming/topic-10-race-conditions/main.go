package main

import (
	"fmt"
	"sync"
)

func updateMessage(s string, wg *sync.WaitGroup, msg *string) {
	defer wg.Done()
	*msg = s
}

func main() {
    msg := "Hello, world"
    var wg sync.WaitGroup

    wg.Add(2)
    go updateMessage("Hello, universe!", &wg, &msg)
    go updateMessage("Hello, cosmos!", &wg, &msg)
    wg.Wait()

    fmt.Println(msg)

    // NOTE: You can check for race conditions in Go by using the command

    // go run -race .

    // This program has a race condition.
    // You have two goroutines trying to access the same data without any
    // system of access.

    // This means that the results of `fmt.Println(msg)` are unpredictable.
}
