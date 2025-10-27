/*

   ABOUT: Pass by reference

   A pointer is simply a variable that stores a memory address.
   It acts as a reference to another variable,
   and the value of that original variable
   can be changed through a process called dereferencing the pointer.

*/

package main

import "fmt"

func main() {
	var value_a int = 11
	fmt.Printf("value_a is %d\n", value_a)
	fmt.Printf("value_a is stored at: %p\n", &value_a)
	// value_a is 11
	// value_a is stored at: 0xc000012110

	// NOTE: How to create a pointer:

	// The variable pointer_a,
	// is a pointer that points to the memory address of an
	// integer (`*int`).

	// So pointer_a is set to the memory address of value_a.
	// In other words, pointer_a holds the location where value_a is stored.

	var pointer_a *int = &value_a

	// TODO: Continue from here and edit everything below this line

	//_________________________________________________________________________

	// You can see that pointer_a matches the memory address of value_a

	fmt.Printf("pointer_a is pointing to: %p\n", pointer_a)
	// pointer_a is pointing to: 0xc000012110

	//_________________________________________________________________________

	// You can dereference a pointer to view the value of the memory address
	// that it is pointing to.

    fmt.Printf("The value at the memory address that pointer_a points to:\n")
	fmt.Printf("is: %d\n", *pointer_a)
    // The value at the memory address that pointer_a points to:
    // is: 11

	//_________________________________________________________________________

    // NOTE: If you change the value of the original, 
    // the value of the pointer_a will change.
   
    value_a = 600;
    println("value_a updated to 600")

    fmt.Printf("The value at the memory address that pointer_a points to:\n")
	fmt.Printf("is: %d\n", *pointer_a)
    // The value at the memory address that pointer_a points to:
    // is: 600

    //_________________________________________________________________________
    
    // NOTE: You can also dereference the pointer to change the value, 
    // the value of of the original.
  
    *pointer_a = 12;
    println("*pointer_a changed to 12")
    
    fmt.Printf("The value at the memory address that pointer_a points to:\n")
	fmt.Printf("is: %d\n", *pointer_a)
    // The value at the memory address that pointer_a points to:
    // is: 12

    fmt.Printf("value_a is %d\n", value_a)
    // value_a is 12

    //_________________________________________________________________________

}
