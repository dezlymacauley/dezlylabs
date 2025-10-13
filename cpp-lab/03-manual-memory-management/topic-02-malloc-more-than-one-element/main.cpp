// This allows you to use `printf` function from the C standard library.
// And size_t to get the size of a variable type in bytes.
#include <cstdio>

// This allows you to use the `malloc`,
// and `free` functions from the C standard library
#include <cstdlib>
#include <cstring>

int main() {
    // This is how you allocate memory for more than
    // one element using malloc
    // So this is a list of 5 integers

    int initial_number_of_elements = 5;
    int initial_value_of_each_element = 0;
    size_t size_of_each_element_in_bytes = sizeof(int);

    int* listOfNumbers = (int*)malloc(initial_number_of_elements *
                                      size_of_each_element_in_bytes);

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

    // NOTE: Unlike calloc which performs memory initialization
    // (Sets the value of every element to 0), malloc does NOT initialize
    // the memory you allocate.
    // So remember to use `memset` when working with `malloc`

    // The syntax is:
    // memset(pointer, initial_value_of_each_element, size_of_each_element);
    // The size of each element is in bytes.

    memset(listOfNumbers, initial_value_of_each_element,
       initial_number_of_elements * size_of_each_element_in_bytes);

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

    for (int i = 0; i < initial_number_of_elements; i++) {
        printf("%d\n", listOfNumbers[i]);
    }

    // 50
    // 0
    // 0
    // 0
    // 0

    //_________________________________________________________________________
    // SECTION: Clean Up

    // Remember to free the memory when it is no longer needed.
    // This will release the memory back to the operating system.
    free(listOfNumbers);

    // To avoid dangling references, set the pointer to `nullptr`
    listOfNumbers = nullptr;

    //_________________________________________________________________________
}
