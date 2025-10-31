/* 

ABOUT: Printing a memory address

*/

#include <print>

using std::println;

int main() {
    int value_a = 12;
    println("value_a is stored at {}", static_cast<void*>(&value_a));
    // value_a is stored at 0x7ffd957a79c4
}
