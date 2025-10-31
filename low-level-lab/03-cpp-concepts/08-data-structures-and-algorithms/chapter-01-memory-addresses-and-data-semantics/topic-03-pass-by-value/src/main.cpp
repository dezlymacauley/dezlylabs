/* 

ABOUT: Pass by value

*/

#include <cstdio>
#include <print>

using std::println;

int main() {
    int variable_a = 12;
    println("variable_a is {}", variable_a);
    printf("variable_a is stored at %p\n", &variable_a);
    // variable_a is 12
    // variable_a is stored at 0x7ffc7f615db4

    // variable_a will get its own copy of the value of variable_b,
    // which will be stored at a different location in memory.
    int variable_b = variable_a;
    println("variable_b is {}", variable_b);
    printf("variable_b is stored at %p\n", &variable_b);
    // variable_b is 12
    // variable_b is stored at 0x7ffc7f615db0
}
