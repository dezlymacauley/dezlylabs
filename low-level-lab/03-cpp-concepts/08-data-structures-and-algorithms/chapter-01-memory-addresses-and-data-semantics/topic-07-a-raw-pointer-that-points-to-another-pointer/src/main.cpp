#include <cstdio>
#include <print>

using std::println;

int main() {
    // This is a pointer to an integer stored on the heap
    int* variable_a = new int(11);
    printf("variable_a stores the memory address: %p\n", variable_a);
    println("variable_a points to the value: {}", *variable_a);
    // variable_a stores the memory address: 0x55789e4c3320
    // variable_a points to the value: 11

    // This is a pointer to another pointer
    // variable_a and variable_b both point
    // to the same location in memory.
    int* variable_b = variable_a;
    printf("variable_b stores the memory address: %p\n", variable_b);
    println("variable_b points to the value: {}", *variable_b);
    // variable_b stores the memory address: 0x55789e4c3320
    // variable_b points to the value: 11

    //_________________________________________________________________________

    // If you update the value at that memory address
    // by dereferencing either variable_a or variable_b,
    // both values will be updated.

    println("=======================");

    *variable_b = 54;

    println("variable_b updated to 54");
    println("variable_a points to the value: {}", *variable_a);
    println("variable_b points to the value: {}", *variable_b);
    // variable_a points to the value: 54
    // variable_b points to the value: 54

    //_________________________________________________________________________
    
    println("=======================");

    *variable_a = 45;

    println("variable_b updated to 45");
    println("variable_a points to the value: {}", *variable_a);
    println("variable_b points to the value: {}", *variable_b);
    // variable_b updated to 45
    // variable_a points to the value: 45
    // variable_b points to the value: 45

    //_________________________________________________________________________
}
