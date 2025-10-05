package main

import (
	"io"
	"os"
	"strings"
	"sync"
	"testing"
)

// NOTE: To run this test run:
// go run .

func Test_printSomething(t *testing.T) {
    stdOut := os.Stdout

    r, w, _ := os.Pipe()
    os.Stdout = w

    var wg sync.WaitGroup
    wg.Add(1)

    go printSomething("Hey there", &wg)

    wg.Wait()

    _ = w.Close()

    result, _ := io.ReadAll(r)
    output := string(result)

    os.Stdout = stdOut

    if !strings.Contains(output, "Hey there") {
        t.Errorf("The output should be: Hey there")
    }
}
