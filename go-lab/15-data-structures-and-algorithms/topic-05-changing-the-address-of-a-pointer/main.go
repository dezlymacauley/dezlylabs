package main

import "fmt"

func main() {
    var value_a int8 = 20;
    fmt.Printf("value_a is stored at: %p\n", &value_a)
    // value_a is stored at: 0xc000012110

    var value_b int8 = 40;
    fmt.Printf("value_b is stored at: %p\n", &value_b)
    // value_b is stored at: 0xc000180000

    var awesome_pointer *int8 = &value_a;
    fmt.Printf("awesome_pointer points to value_a: %p\n", awesome_pointer)
    // awesome_pointer points to value_a: 0xc000012110
    
    awesome_pointer = &value_b;
    fmt.Printf("awesome_pointer now points to value_b: %p\n", awesome_pointer)
    // awesome_pointer now points to value_b: 0xc000180000
}
