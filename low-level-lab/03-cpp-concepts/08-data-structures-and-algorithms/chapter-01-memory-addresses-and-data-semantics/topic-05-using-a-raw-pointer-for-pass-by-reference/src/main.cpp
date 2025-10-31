/*

ABOUT: Using a raw pointer for pass by reference

*/

// for `std::println`
#include <print>

// for `printf`
#include <cstdio>

using std::println;

int main() {

    int variable_a = 12;
    println("variable_a is: {}", variable_a);
    printf("variable_a is stored at: %p\n", &variable_a);
    // variable_a is: 12
    // variable_a is stored at: 0x7fffe3700724

    int* pointer_to_a = &variable_a;
    printf("pointer_to_a stores the memory address: %p\n", pointer_to_a);
    println("pointer_to_a points to the value: {}", *pointer_to_a);
    // pointer_to_a stores the memory address: 0x7fffe3700724
    // pointer_to_a points to the value: 12

    // NOTE: Do not use the `delete` keyword to free stack-allocated memory
    // here or the program will crash because stack-allocated memory
    // is automatically managed by C++

    // In this specific example,
    // pointer_to_a is pointing to a memory location that is stored on the
    // stack. variable_a is stored on the stack.

    // The delete keyword is for freeing heap-allocated memory.

    // WARNING: Don't forget about the cleanup when using raw pointers
    pointer_to_a = nullptr;
}
