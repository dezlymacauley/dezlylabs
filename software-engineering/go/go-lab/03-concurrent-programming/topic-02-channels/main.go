/*

ABOUT:

So basically this concept is sort of like Rust's JoinHandle,
except rust gives you one when you spawn a thread. 

In Go you have to create one manually and then add it to the function.

*/
package main

import (
	"fmt"
	"time"
)

func receivePizza(deliveryStatus chan bool) {
    // This is used to simulate a real-world delay
    time.Sleep(2 * time.Second)
   
    fmt.Println()
    fmt.Println("Hey it's me. The pizza delivery guy")

    // This will send a bool value of `true` to `doneChan` in `func main()`
    // so that `func main()` knows when this function is done.
    deliveryStatus <- true
}

func main() {

    // This is a channel (think of it as a communication device).
    // It allows a function in a goroutine to send some data back 
    // to `func main`. 
    // In this case the `receivePizza` function will be sending a bool back,
    // to inform `func main()` that the `receivePizza` function is done.
    pizzaDeliveryStatus := make(chan bool)
  
    println("I've just ordered some Pizza. Now we wait")
    
    go receivePizza(pizzaDeliveryStatus)
    
    println("Putting the glasses on the table.")
    println("Pouring the drinks.")

    // This reads a value from the channel.
    // `main()` will block here until `receivePizza` sends a value,
    // signaling that the pizza has been delivered.

    // NOTE: The bool value from the channel is being read 
    // and then discarded. That is why the left side is empty.
    // But can read the into a variable if you wanted to.
    <- pizzaDeliveryStatus

    // This message will not display until the boolean value has 
    // been received from the channel.
    println("Time to start the party")
}
