package main

import (
	"fmt"
	"runtime"
	"unsafe"
)

func main() {
    // This will create a memory address that it large enough 
    // to store a 8 bit integer value that will be stored on the Heap.

    // The value of value_c will be set to 0.
    // The reason why it's set to 0 is because Go will use the zero value
    // of the variable type that you want it to create a memory address for.
    var value_c *int8 = new(int8)
    fmt.Printf("value_c is stored at: %p\n", value_c)
    // value_c is stored at: 0xc000012110

    fmt.Printf("The value of value_c is: %d\n", *value_c) 
    // The value of value_c is: 0

    // To update the value
    *value_c = 40
    fmt.Printf("The value of value_c is now %d\n", *value_c)
    // The value of value_c is now 40

    // To see how much space in memory a variable is taking up:
    // Use `v` which is the generic format specifier
    fmt.Printf("The value of value_c is:\n")
    fmt.Printf("%v byte in memory\n", unsafe.Sizeof(*value_c))     
    // The value of `value_c` is:
    // 1 byte in memory

    // 1 byte = 8 bits

    // NOTE: Always make sure that you use:
    // `unsafe.Sizeof(*value_c)` to get the size of the actual value.
    
    // If you leave out the `*` and use:
    // `unsafe.Sizeof(value_c)` you will get the size of the pointer,
    // which is either 32 bits or 64 bits, 
    // depending on the CPU architecture that is running the Go program.

    // NOTE: To check what your CPU architecture is:

    fmt.Printf("Your CP architecture is: %v\n", runtime.GOARCH)
    
    // NOTE: To check what the pointer size on that architecture is:

    println("The pointer size on this architecture is:")
    // Your CP architecture is: amd64

    fmt.Printf("%v bits\n", 8*unsafe.Sizeof(uintptr(0)))
    // The pointer size on this architecture is:
    // 64 bits

}
