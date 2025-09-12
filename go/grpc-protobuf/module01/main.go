package main

import (
	"log"

	"github.com/dezlymacauley/module1/proto"
)

func main() {
    person := proto.Person{
        Name: "Dezly",
    }

    // Protobuff will generate a getter function for each property on the
    // message definition.
    //
    // This function has a nillcheck.
    // if person is nil, then `GetName` will return an empty string.
    log.Println(person.GetName());
}

// go run main.go
// 2025/09/12 15:40:09 Dezly
