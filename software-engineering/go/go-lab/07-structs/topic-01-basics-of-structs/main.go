package main

import (
	"fmt"
	"time"
)

// NOTE: Please note that the casing of the first letter matters in Go

// lowercase means the struct is private. I.e. Can only be accessed from
// this file.

// Uppercase means the struct is public. I.e. It can be accessed outside of
// this file.

// type NameOfStruct struct {}
type user struct {
	firstName string
	lastName  string
	birthDate string
	createdAt time.Time
}

func getUserData(promptText string) string {
	var value string = ""
	fmt.Print(promptText)

	fmt.Scan(&value)
	return value
}

func outputUserDetails(firstName string, lastName string, birthDate string) {
	fmt.Println(firstName, lastName, birthDate)
}

func main() {
	userFirstName := getUserData("Please enter your first name: ")
	userLastName := getUserData("Please enter your last name: ")
	userBirthdate := getUserData("Please enter your birthdate (MM/DD/YYYY): ")

	// NOTE: This is how to create an instance of a struct

    // When you declare an instance of a struct like this, 
    // the right side of the equals sign is called a `composite literal`,
    // or to be more specific a `struct literal`

    // The term `composite literal` is a general term in Go.
    // It refers to expressions like T{...}, where T can be
    // an array, slice, map, or struct type.
    var appUser user = user{
       firstName: userFirstName,
       lastName: userLastName,
       birthDate: userBirthdate,
       createdAt: time.Now(),
    }

	outputUserDetails(userFirstName, userLastName, userBirthdate)
}
