/*
    ABOUT: The advantage of `new` 

    You can allocate memory and initialize it in the same line.

    The difference between `maloc` and the `new` keyword.

    Malloc:
    Is a function.
    Requires size during allocation
    Cannot initialize memory
    Malloc can't be used to create objects because it can't 
    call constructors.
    Malloc returns a void pointer that needs to be type casted.
    Cannot be customized.
    Returns null when it fails.

    New:
    Is an operator.
    It will automatically set the size based on the type.
    It can intialize memory
    It can call constructors.
    Returns the correct type of the pointer.
    New can be customized through overloading.
    Throws exception on failure.

    NOTE: In C++ you should always used `new` instead of malloc,
    and you should always use `delete` instead of `free`

    free is the C version.

    delete is the C++ version and it has the ability to call the destructor
    of the object.

*/

#include <cstdio>
int main() {
    // The value 6 will be stored in Heap Memory.
    int* userAge = new int(6);
    printf("userAge is: %d\n", *userAge);
    // userAge is: 6

    // NOTE: Clean up is still required
    
    delete userAge;
    userAge = nullptr;
}
