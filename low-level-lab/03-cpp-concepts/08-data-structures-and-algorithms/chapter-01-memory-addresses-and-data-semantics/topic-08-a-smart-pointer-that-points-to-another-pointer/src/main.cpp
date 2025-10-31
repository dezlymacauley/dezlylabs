#include <cstdio>
#include <memory>
using std::make_shared;
using std::shared_ptr;

#include <print>
using std::println;

int main() {

    // This creates a shared pointer in heap memory that is large enough
    // to store and integer value, and then stores the number 11 there.

    // Unlike a std::unique_ptr (which has exclusive ownership 
    // of the heap-allocated memory, and the data stored there),
    // a std::shared_ptr allows multiple pointers to share ownership of
    // the same memory location and modify the value stored there.

    // A shared_ptr will automatically free the memory when then pointer
    // goes out of scope AND there are no other pointers pointing to it.
    // The memory allocated is automatically freed when
    // the last shared_ptr that owns it goes out of scope.

    // In simple terms, when the reference count reaches zero.
    shared_ptr<int> pointer_to_int_value = make_shared<int>(11);

    // TODO: Continue Here

    // printf("pointer_to_int_value stores the memory adress: %p\n",
    //        pointer_to_int_value.get());

    // println("pointer_to_int_value points to the value: {}",
    //         *pointer_to_int_value);

    // This creates a shared pointer,
    // that points to the same memory location as `pointer_to_int_value`
    // shared_ptr<int> spiderman_pointer =

    // println("variable_a points to the value: {}", *variable_a);
    //
    // // Create another shared pointer that points to the same memory
    // auto variable_b = variable_a;
    // println("variable_b points to the value: {}", *variable_b);
    //
    // println("=======================");
    //
    // // Update through variable_b
    // *variable_b = 54;
    // println("variable_b updated to 54");
    // println("variable_a points to the value: {}", *variable_a);
    // println("variable_b points to the value: {}", *variable_b);
    //
    // println("=======================");
    //
    // // Update through variable_a
    // *variable_a = 45;
    // println("variable_a updated to 45");
    // println("variable_a points to the value: {}", *variable_a);
    // println("variable_b points to the value: {}", *variable_b);
}
