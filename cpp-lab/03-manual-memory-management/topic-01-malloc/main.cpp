// This allows you to use `printf` function from the C standard library.
#include <cstdio>

// This allows you to use the `malloc`, 
// and `free` functions from the C standard library
#include <cstdlib>

int main() {

    // `malloc` is a function that is used to manually allocate memory.
    // It accepts the size of the memory you want to allocate,
    // and it returns a pointer to the memory address 
    // that has been allocated.
    // The variable type of the pointer returned by malloc is `void*`.
    // `void*` is basically a generic pointer.
    // You can convert `void*` into the specific type of pointer you want
    // by type casting it. 
    // You do that by placing (variableType*)
    int* userAge = (int*) malloc(sizeof(int));

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
