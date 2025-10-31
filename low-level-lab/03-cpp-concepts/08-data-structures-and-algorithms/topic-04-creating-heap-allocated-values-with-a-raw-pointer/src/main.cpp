/* 

ABOUT: Creating Heap-Allocated Values With A Raw Pointer

*/

#include <cstdio>

int main() {
    // pointer_to_a is of the type `int*`
    // `int*` means a pointer an integer value
    // variable_a is a pointer, which means that it stores a memory address,
    // on the stack, while the value 12 is stored in heap memory.

    // In simple terms, the pointer is stored on the stack
    // but the value it points to is stored on the heap  
    // via the `new T()` syntax.
    // This will reserve a location in memory that is big enough to store
    // an int variable, and then the value of 12 will be stored there.
    int* pointer_to_a = new int(12);
    printf("pointer_to_a stores the memory address: %p\n", pointer_to_a);
    // pointer_to_a stores the memory address: 0x56456f626320

    //_________________________________________________________________________

    // To access the value of a pointer you need to 
    // use this `*`. This is known as the dereference operator.

    printf("pointer_to_a points to the value: %d\n", *pointer_to_a);
    // pointer_to_a points to the value: 12

    //_________________________________________________________________________
    
    // WARNING: Don't forget about cleanup when using raw pointers

    // Now the downside to using a raw pointer is that you have to remember
    // to free the memory that the pointer is pointing to.

    // And you must set the pointer to `nullptr` to indicate that it no
    // longer points to a valid location in memory.
    // This is done to prevent dangling references.

    delete pointer_to_a;
    pointer_to_a = nullptr;
}
