/*

    ABOUT: Pass by value

    value_b will get its own copy of the value of value_a.

    So changing the the value of value_b will NOT change the value of
    value_a.

*/ 

package main

import "fmt"

func main() {
    var value_a int = 11;
    fmt.Printf("value_a is %d\n", value_a);
    fmt.Printf("value_a is stored at: %p\n", &value_a)
    // value_a is 11
    // value_a is stored at: 0xc000012110

    var value_b int = value_a;
    fmt.Printf("value_b is %d\n", value_b);
    fmt.Printf("value_b is stored at: %p\n", &value_b)
    // value_b is 11
    // value_b is stored at: 0xc000182000

    value_b = 22;
    println("value_b has been updated to 22")
    println("==============================")
    
    fmt.Printf("value_a is %d\n", value_a);
    // value_a is 11

    fmt.Printf("value_b is %d\n", value_b);
    // value_b is 22
}
