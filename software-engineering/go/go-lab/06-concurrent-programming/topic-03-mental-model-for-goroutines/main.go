/*

ABOUT: Understanding goroutines and channels

_______________________________________________________________________________c
1. The mental model required to understand goroutines and channels:

Think of a Go program as a spaceous office building where tasks can be 
performed in separate office rooms.

There will always be work being performed in at least one room,
but the usage of the other office rooms is completely up to you.

If you don't specify that you want to use more than one office room, 
all of the tasks will be performed in a single office room.

Each office room must have at least one appointed office manager, 
that will communicate to the General Manager 
or in some cases another office manager.
_______________________________________________________________________________

2. What is a goroutine:

Think of a goroutine as an office room where work is being performed.

`func main()` is actually a goroutine, 
and this office room is managed by the Go scheduler.

Think of the Go scheduler as the General Manager of the company.

NOTE: And import difference to note between a goroutine in Go,
versus a thread in the Rust programming language.

Unlike Rust, a goroutine is NOT the built-in hardware threads of a processor.
A goroutine is a very lightweight thread.
These are threads that are specific to Go, and managed by the Go scheduler.
The Go scheduler decides when each goroutine runs,
and how much processing time it gets.

_______________________________________________________________________________

3. How to use the `go` keyword to start 
an additional goroutine (i.e. make use of more than one office room)

You simply need to put the `go` keyword in front of a function when you call
the function
   
E.g. I create this function

func slowGreet(phrase string) {
    time.Sleep(3 * time.Second)
    fmt.Println("Hello", phrase)
}

Then I call it like this:

go slowGreet("How ... are ... you ...?")

This tell Go to perform this task in another office room. That way any
tasks in the main goroutine `func main()` can progress while 
`slowGreet` completes.

_______________________________________________________________________________c

4. Adding the office manager (aka a channel)

`func main()` is a special goroutine. It does not need the `go` keyword
in front of it and it already has a General Manager (The Go scheduler).

For `slowGreet` this is not the case. This office now needs an office manager,
that will tell the General Manager when slowGreet is done.

go slowGreet("How ... are ... you ...?")

Step 1: This is how to create an "office manager"

The syntax is name := make(chan variable type of status update 
or data that is sent back by the manager)

So think of this as a status update. In this case it will return 
true or false.

done := make(chan bool)

Step 2: Add the channel (office manager), to the goroutine (the office)

func slowGreet(phrase string, doneChan chan bool) {
    time.Sleep(3 * time.Second)
    fmt.Println("Hello", phrase)
    doneChan <- true
}

_______________________________________________________________________________

Step 3: Call the function

go slowGreet("How ... are ... you ...?", done)

_______________________________________________________________________________

*/

package main

import (
	"fmt"
	"time"
)

func greet(phrase string) {
    fmt.Println("Hello!", phrase)
}

func slowGreet(phrase string, doneChan chan bool) {
    time.Sleep(3 * time.Second)
    fmt.Println("Hello", phrase)
    // Let the office manager know that the work is done.
    doneChan <- true
}

func main() {
    // go greet("Nice to meet you!")
    // go greet("How are you")
   
    // The variable `done` is of the type `chan bool`
    done := make(chan bool)

    go slowGreet("How ... are ... you ...?", done)
    // go greet("I hope you are liking the course")

    // Block the main goroutine until you get a response back 
    // from the channel (aka the office manager) that the work is done.
    <- done
}
