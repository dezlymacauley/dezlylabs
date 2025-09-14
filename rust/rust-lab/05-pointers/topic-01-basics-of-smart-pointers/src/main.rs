/*

ABOUT: Smart Pointer

A smart pointer contains a memory address and additional meta data.

1. Box Smart Pointer:
Provides a way to reference data that is stored on the Heap.

*/

fn main() {
    //_________________________________________________________________________
    // SECTION: Box Smart Pointer (Owns data)

    // This is a type of Smart Pointer.

    // This variable will point to a value of 0.625 
    // that is stored on the Heap.
    // The variable itself `single_value` will be stored on the stack.
    let num_one: Box<i32> = Box::new(21);

    // This is stored on the Stack
    let num_two: i32 = 9;

    // You need to use `*` to dereference num_two so 
    // that you can get its value.
    let total: i32 = *num_one + num_two;
    println!("total: {total}");

    //_________________________________________________________________________
    // SECTION: Reference Pointer (Borrows data that it points to)

    // This is NOT a smart pointer because it only stores 
    // the memory address.

    // Stored on the stack
    let a: i32 = 20;

    // `ref_a` is a variable that stores the memory address of a variable 
    // that is stored on the stack.
    // The variable `ref_a` itself is stored on the heap.
    let ref_a: &i32 = &a;

    //_________________________________________________________________________
   
    
    let stack_var: i32 = 4;

    let heap_var: Box<i32> = Box::new(stack_var);


    //_________________________________________________________________________
}
