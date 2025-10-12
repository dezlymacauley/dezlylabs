// This allows you to use `printf` function from the C standard library.
#include <cstdio>

// This allows you to use the `calloc`, 
// and `free` functions from the C standard library
#include <cstdlib>

int main() {

    // `calloc` is a function that is used to manually allocate memory.
    // It accepts:
    // 1. The number of elements that you want to allocate memory for.
    // 2. The size of the memory you want to allocate,
    // and it returns a pointer to the memory address 
    // that has been allocated.

    // The variable type of the pointer returned by calloc is `void*`.
    // `void*` is basically a generic pointer.
    // You can convert `void*` into the specific type of pointer you want
    // by type casting it. 
    // You do that by placing (variableType*)
    
    int* userAge = (int*) calloc(1, sizeof(int));

    // Always check if the memory allocation was successful
    if (userAge == nullptr) {
        printf("Memory allocation failed!\n");
        return 1;
    }

    // `userAge` is a pointer (a memory address)
    // So to access the value that `userAge` points to, 
    // you must deference it.
    *userAge = 50;

    printf("userAge is: %d\n", *userAge);
    // userAge is: 50

    //_________________________________________________________________________
    // SECTION: Clean Up

    // Remember to free the memory when it is no longer needed.
    // This will release the memory back to the operating system.
    free(userAge);

    // To avoid dangling references, set the pointer to `nullptr`
    userAge = nullptr;
    
    //_________________________________________________________________________
}
