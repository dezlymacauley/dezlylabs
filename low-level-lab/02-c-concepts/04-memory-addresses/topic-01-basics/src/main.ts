/*

#include <stdio.h>

int main() {
    int value_a = 11;
    printf("value_a is %d\n", value_a);
    printf("value_a is stored at: %p\n", (void*)&value_a);
    // value_a is 11
    // value_a is stored at: 0x7fff3d8ca500

    int value_b = value_a;
    printf("value_b is %d\n", value_b);
    printf("value_b is stored at: %p\n", (void*)&value_b);
    // value_a is 11
    // value_b is stored at: 0x7fff3d8ca504
}

*/

let value_a: number = 11;
console.log(`value_a is ${value_a}`);
// value_a is 11

let value_b: number = value_a;
console.log(`value_b is ${value_b}`);
// value_b is 11
