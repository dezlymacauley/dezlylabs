package main

import (
	"fmt"
    "sync"
)

func printSomething(s string, wg *sync.WaitGroup) {
	fmt.Println(s)
    defer wg.Done()
}

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

    // WARNING: Never add a hardcoded value
    // Hardcoded values can lead to bugs and crashes if you get the value
    // wrong.
    wg.Add(len(shoppingList))

    for index, value := range shoppingList{
        go printSomething(fmt.Sprintf("%d: %s", index, value), &wg)
    }

    // This tells the main goroutine to wait until all of the goroutines
    //  in the Wait Group have finished processing.
    wg.Wait()
   
    // This line will only be printed once the last goroutine 
    // in the WaitGroup is done.
    fmt.Println("Let's go!!!")

    // NOTE: Please note that in this specific scenario where a goroutine
    // is created each time a `for range` loop runs,
    // the order that the messages are printed will be different each time
    // the loop runs.
    //
    // 4: bread
    // 0: apple
    // 1: cake
    // 2: pizza
    // 3: grape
    // Let's go!!!

}
