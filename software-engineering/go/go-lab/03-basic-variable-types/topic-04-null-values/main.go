package main

import "fmt"

/*

All Go value types have a null-value which is automatically
assigned to to variable if you declare a variable without explicitly
setting the value.

I generally avoid doing this because it's bad practice.

*/

func main() {

    var myNum int
    var accountBalance float64
    var myMessage string 
    var isOnline bool

    println(myNum) // 0
    println(accountBalance) // +0.000000e+000
    println(myMessage) // null-value is an empty string ""
    println(isOnline) // false

    fmt.Println(accountBalance) // 0
}
