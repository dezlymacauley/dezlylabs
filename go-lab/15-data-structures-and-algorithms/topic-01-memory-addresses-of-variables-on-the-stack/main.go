package main

import "fmt"

func main() {
    // num1 is a variable that is created on the stack
    var num1 int = 11;
    fmt.Printf("num1 is %d\n", num1);
    fmt.Printf("num1 is stored at: %p\n", &num1)
    // num1 is 11
    // num1 is stored at: 0xc000012110
}
