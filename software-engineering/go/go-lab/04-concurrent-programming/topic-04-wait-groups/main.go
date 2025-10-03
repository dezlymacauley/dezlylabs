package main

import (
	"fmt"
    "sync"
)

func printSomething(s string, wg *sync.WaitGroup) {
	fmt.Println(s)
    defer wg.Done()
}

// The main function in Go is a goroutine
// A goroutine is a very lightweight thread.
// This is not the built-in hardware threads of a processor.
// These are threads that are specific to Go, and managed by the Go scheduler.
// The Go scheduler decides when each goroutine runs,
// and how much processing time it gets.
func main() {

    // A wait group is like a counter,
    // it keeps track of how many goroutines are running.
    var wg sync.WaitGroup

	shoppingList:= []string{
		"apple", "cake", "pizza", "grape", "bread",
	}

    // If you look at the `for range` loop below, 
    // you will see that that for each value (each word) in the 
    // `sshoppingList` array.
    // wg.Add() accepts an integer amount, which is the number of goroutines
    // that you intend to start.

    // I'm using `len(shoppingList)` to get the number of items in the
    // shoppingList array.

    // Please note that this must be set before the goroutines start.
    wg.Add(len(shoppingList))

    for index, value := range shoppingList{
        go printSomething(fmt.Sprintf("%d: %s", index, value), &wg)
    }

    wg.Wait()

    wg.Add(1)
	printSomething("This is the second message to be printed", &wg)


// 4: e
// 2: c
// 0: a
// 1: b
// 3: d
// This is the second message to be printed

}
