/*

ABOUT: Using a raw pointer to create Heap-allocated values

*/

#include <cstdio>

int main() {
    // pointer_to_int_value is of the type `int*`
    // `int*` means a pointer an integer value

    // pointer_to_int_value is a pointer,
    // which means that it stores a memory address,
    // on the stack, while the value 12 is stored in heap memory.

    // In simple terms, the pointer is stored on the stack
    // but the value it points to is stored on the heap
    // via the `new T()` syntax.

    // This will reserve a location in memory that is big enough to store
    // an int variable, and then the value of 12 will be stored there.
    int* pointer_to_int_value = new int(12);
    printf("pointer_to_int_value stores the memory address: %p\n",
           pointer_to_int_value);
    // pointer_to_int_value stores the memory address: 0x55e9fb5ff320

    //_________________________________________________________________________

    // To access the value of a pointer you need to
    // use this `*`. This is known as the dereference operator.

    printf("pointer_to_int_value: %d\n", *pointer_to_int_value);
    // pointer_to_int_value: 12

    //_________________________________________________________________________

    // WARNING: Don't forget about cleanup when using raw pointers

    // Now the downside to using a raw pointer is that you have to remember
    // to free the memory that the pointer is pointing to.

    // And you must set the pointer to `nullptr` to indicate that it no
    // longer points to a valid location in memory.
    // This is done to prevent dangling references.

    delete pointer_to_int_value;
    pointer_to_int_value = nullptr;
}
