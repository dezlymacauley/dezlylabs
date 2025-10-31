/*

ABOUT: Using a smart pointer to create Heap-allocated values

A smart pointer is a wrapper class around a raw pointer 
that adds automatic memory management.

*/

// Needed for using `printf()`
#include <cstdio>

// Needed for ``
#include <memory>

using std::make_unique;
using std::unique_ptr;

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

    printf("pointer_to_int_value: %d\n", *pointer_to_int_value);
    // pointer_to_int_value: 12

    // Not that `*pointer_to_int_value`, 
    // is shorthand for this: `pointer_to_int_value.operator*()`

    // NOTE: No clean up is needed when using smart pointer

}
