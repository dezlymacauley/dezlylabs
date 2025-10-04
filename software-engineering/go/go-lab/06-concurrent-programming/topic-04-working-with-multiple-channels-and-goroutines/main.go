package main

import (
	"fmt"
	"time"
)

func greet(phrase string, doneChan chan bool) {
    fmt.Println("Hello!", phrase)
    doneChan <- true
}

func slowGreet(phrase string, doneChan chan bool) {
    time.Sleep(3 * time.Second)
    fmt.Println("Hello", phrase)
    doneChan <- true
}

func main() {

    // In this case you have a channel (an office manager) that is managing
    // multiple offices.
    done := make(chan bool)

    go greet("Nice to meet you!", done)
    go greet("How are you", done)
    go slowGreet("How ... are ... you ...?", done)
    go greet("I hope you are liking the course", done)

    // You would have to read from the channel 4 time.
    // One for each completed goroutine.
    <- done
    <- done
    <- done
    <- done
}
