package main

import (
	"fmt"
)

func getUserData(promptText string) string {
    var value string = ""
    fmt.Print(promptText)
    
	fmt.Scan(&value)
	return value
}

func main() {
	firstName := getUserData("Please enter your first name: ")
	lastName := getUserData("Please enter your last name: ")
	birthdate := getUserData("Please enter your birthdate (MM/DD/YYYY): ")
	fmt.Println(firstName, lastName, birthdate)
}
