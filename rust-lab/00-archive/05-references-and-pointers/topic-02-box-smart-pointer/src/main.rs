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

    // This variable `user_age` will take ownership of some memory on the
    // HEAP, to store the value of 21 on the HEAP.

    // The variable itself `user_age` will be stored on the STACK.
    let user_age: Box<i32> = Box::new(21);

    println!("value of user_age: {}", *user_age);

    println!("memory address of user_age: {user_age:p}");
    // value of user_age: 21
    // memory address of user_age: 0x562aa5445d70

    //_________________________________________________________________________
    // SECTION: Example 2 

   
    let stack_var: i32 = 4;

    // `Box::new()` will allocate memory on the Heap 
    // for an i32 variable type.
    //
    // Then a COPY of the value of `stack_var`, which is 4 is copied to this
    // memory location. 
    //
    // The reason a copy happens is because i32 implements the Copy trait. 
    // All primitive scalar types (like integers, booleans, floats, 
    // and chars) implement Copy.
    //
    // The variable `heap_var` itself is stored on the stack.
    // `heap_var` is a pointer (a memory address) to the location 
    // on the HEAP where 4 is stored.
    let mut heap_var: Box<i32> = Box::new(stack_var);
    println!("heap_var: {}", *heap_var);

    // NOTE: To change the value of `heap_var` you need to dereference it
    // to get the value that it is pointing to.

    *heap_var = 16;
    println!("heap_var: {}", *heap_var);
    // heap_var: 16

    println!("stack_var: {stack_var}");
    // stack_var: 4

    // The original value of stack_var will not be changed becase this was
    // a Copy and not a move.

    //_________________________________________________________________________
}
