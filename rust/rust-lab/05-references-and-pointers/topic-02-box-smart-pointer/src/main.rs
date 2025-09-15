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
}
