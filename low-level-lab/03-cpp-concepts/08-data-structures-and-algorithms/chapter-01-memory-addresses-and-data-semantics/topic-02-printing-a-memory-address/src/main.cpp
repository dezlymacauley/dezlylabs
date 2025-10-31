/* 

ABOUT: Printing a memory address

For printing out a memory address, 
I prefer using `printf` from the C standard library.

The usage is also identical to fmt.Printf() in the Go programming language.

*/

#include <cstdio>

int main() {
    int variable_a = 12;

    // `%p` is the format specifier for a pointer.
    // This tells printf that you want to print a pointer, 
    // which is memory address.
    // `&` symbol before the variable name is called an ampersand.
    // &variable_a means "memory address of variable_a"
    printf("variable_a is stored at %p\n", &variable_a);
    // variable_a is stored at 0x7ffc185e7254

    // A memory address is usually shown in hexadecimal format.
    // The prefix `0x` indicates that it is a hexadecimal value.
    // The number of digits depends on your system:
    // 64-bit systems usually show 16 hexadecimal digits (after `0x`).
    // 32-bit systems usually show 8 hexadecimal digits.
}
