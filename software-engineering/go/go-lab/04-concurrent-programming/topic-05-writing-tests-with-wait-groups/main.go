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

    // NOTE: I have deliberately created an error here.
   
    // There are only 5 items in the shoppingList,
    // and the `for range` loop will create 5 go routines.
    // This will lead to a deadlock (when a goroutine is 
    // blocked indefinately).
    // 
    // The deadlock will happen when you call `wg.Wait()` 
    //
    // wg.Wait() will block the main thread until it gets the signal that
    // all goroutines are done. I.e. wg = 0.
    // This will never happen because the sixth goroutine does not exist,
    // and therefore, wg will be stuck at `wg = 1` forever, 
    // which will block the goroutine `func main()`
    wg.Add(6)

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
