/*
    ABOUT: The advantage of `new` 

    You can allocate memory and initialize it in the same line.
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
