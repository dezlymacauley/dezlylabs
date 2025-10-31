package main
import "fmt"

func main() {
    //_________________________________________________________________________
    // SECTION: String 

    // Unlike Rust, Go only has one string type.

    // Like Rust, a contents of string in Go are also stored on the Heap.

    // When you declare a variable as a string in Go, 
    // This is how strings workd under the hood. 

    // When you declare userName, the contents ("Jane") are immutable.
    // This ensures data integrity.
    var userName string = "Jane"

    fmt.Println(userName)
    fmt.Println("User name is", userName)
    fmt.Printf("User name is %s\n", userName)

    // When you do this, you are not modifying the string "Jane"
    // You are actually creating a new string called "Cassie", 
    // and then assigning that string to userName.
    userName = "Cassie"

    fmt.Println(userName)
    fmt.Println("User name is", userName)
    fmt.Printf("User name is %s\n", userName)

	//_________________________________________________________________________
	// SECTION: Bool

	// true, false

    var isOnline bool = true
    fmt.Println(isOnline)
    fmt.Println("Online status is", isOnline)

    // NOTE: For `Printf` you use `%t` to print a bool value
    fmt.Printf("Online status is %t\n", isOnline)

    isOnline = false
	fmt.Println(isOnline)

	//_________________________________________________________________________
}
