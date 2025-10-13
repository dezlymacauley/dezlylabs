// This allows you to use `printf` function from the C standard library.
#include <cstdio>

// This allows you to use the `calloc`, 
// and `free` functions from the C standard library
#include <cstdlib>

int main() {
    // This is how you allocate memory for more than 
    // one element using calloc 
    // So this is a list of 5 integers
    
    int numberOfElements = 5;
    int* listOfNumbers = (int*) calloc(numberOfElements, sizeof(int));

    // Always check if the memory allocation was successful

    // This means "check if this memory adress is a nullpointer". 
    // /That is why you use listOfNumbers and not *listOfNumbers
    // A nullptr is a special constant in C++ that means
    // "This pointer points to nothing. So it is not a valid memory address
    // that you can deference to get a value."
    // Trying to deference a nullptr will cause the program 
    // to crash immediately.
    if (listOfNumbers == nullptr) {
        printf("Memory allocation failed!\n");

        // This is an early return. If memory allocation failed,
        // the function `main` will exit with a status code of `1` and the
        // any lines of code after `return 1` will not be executed.
        return 1;
    }

    // NOTE: When you use calloc to allocate memory, unlike malloc,
    // calloc will perform memory initialization (Set the value of every
    // element to 0).

    // `listOfNumbers` is a pointer (a memory address).
    // More specifically, it is a pointer to the first element in the list.
    // So to access the value that `userAge` points to, 
    // you must deference it.

    // I want to change the first value in the list (index 0) to 50 

    // NOTE: The reason why you use listOfNumbers and not *listOfNumbers
    // here is because when you use the `array subscript operator`, 
    // array[index], C++ will automatically dereference for you.
    // is already done for you to access the value.
    // -------------------------------------------------------------------
    // `array[index] = value_to_set;` 
    // is equal to `*(array + index) = value_to_set;`
    
    listOfNumbers[0] = 50;

    for (int i = 0; i < numberOfElements; i++) {
        printf("%d\n", listOfNumbers[i]);
    }

    // userAge is: 50

    //_________________________________________________________________________
    // SECTION: Clean Up

    // Remember to free the memory when it is no longer needed.
    // This will release the memory back to the operating system.
    free(listOfNumbers);

    // To avoid dangling references, set the pointer to `nullptr`
    listOfNumbers = nullptr;
    
    //_________________________________________________________________________
}
