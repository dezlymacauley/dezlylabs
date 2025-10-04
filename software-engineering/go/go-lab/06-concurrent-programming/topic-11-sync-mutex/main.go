package main

import (
	"fmt"
	"sync"
)

var msg string
var wg sync.WaitGroup

func updateMessage(s string, m *sync.Mutex) {
    defer wg.Done()

    m.Lock()
    msg = s
    m.Unlock()
}

func main() {
    msg = "Hello, world"

    var mutex sync.Mutex

    wg.Add(2)
    go updateMessage("Hello, universe!", &mutex)
    go updateMessage("Hello, cosmos!", &mutex)
    wg.Wait()

    fmt.Println(msg)

    // NOTE: You can check for race conditions in Go by using the command

    // go run -race .

    // There should be no data race now

}
