/*

ABOUT: Using a smart pointer to create new Heap-allocated values

A smart pointer is a wrapper class around a raw pointer
that adds automatic memory management.

*/

// Needed for using `printf()`
#include <cstdio>

// Needed for ``
#include <memory>
#include <print>

using std::make_unique;
using std::unique_ptr;

using std::println;

int main() {

    //_________________________________________________________________________

    unique_ptr<int> pointer_to_int_value = make_unique<int>(12);

    // NOTE: Another syntax

    // You can use `auto` to let C++ infer the variable type `<T>` from
    // `make_unique<T>`
    // auto pointer_to_int_value = make_unique<int>(12);

    //_________________________________________________________________________

    // NOTE: `.get()`

    // When using printf, the format specifier `%p` expects a raw pointer.
    // So to get the the underlying raw pointer of a smart pointer,
    // you have to use the `.get()` method.

    printf("pointer_to_int_value stores the memory address: %p\n",
           pointer_to_int_value.get());
    // pointer_to_int_value stores the memory address: 0x5637507e5320

    //_________________________________________________________________________

    // To access the value of a smart pointer you need to
    // use this `*`. This is known as the dereference operator.

    println("pointer_to_int_value points to the value: {}",
            *pointer_to_int_value);
    // pointer_to_int_value points to the value: 12

    // Not that `*pointer_to_int_value`,
    // is shorthand for this: `pointer_to_int_value.operator*()`

    // NOTE: No clean up is needed when using smart pointer

    //_________________________________________________________________________

    // WARNING: 
    // 1. make_unique<T> will always create new memory,
    // with a new value at that memory location.
    // 2. make_unique<T> will always be pass by reference

    // If you try to pass in a variable to the initializer,
    // it will simply copy the value of that variable,
    // and then store it at a new location in memory.

    // So never try to use make_unique<T> for pass by reference
    // or you will get incorrect results.

    int value_b = 50;
    println("value_b is: {}", value_b);
    printf("value_b is stored at: %p\n", &value_b);
    // value_b is: 50
    // value_b is stored at: 0x7fffea4911b8

    unique_ptr<int> pointer_to_value_b = make_unique<int>(value_b);
    printf("pointer_to_value_b stores the memory address: %p\n",
           pointer_to_value_b.get());
    println("pointer_to_value_b points to the value: {}", *pointer_to_value_b);
    // pointer_to_value_b stores the memory address: 0x564199182790
    // pointer_to_value_b points to the value: 50

    *pointer_to_value_b = 100;
    println("pointer_to_value_b points to the value: {}", *pointer_to_value_b);
    // pointer_to_value_b points to the value: 100

    println("value_b is: {}", value_b);
    // value_b is: 50

    // So this is `pass by value`, and NOT `pass by reference`

    //_________________________________________________________________________
}
