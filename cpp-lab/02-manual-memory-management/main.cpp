#include <iostream>

using std::cout;

int main() {
    int first_number = 18;
    int* ptr_first_number = &first_number;

    cout << "first_number: " << first_number << "\n";

    // Remember to dereference the pointer to access the value at the memory
    // address that it is storing.
    // You do that by placing the `*` in front of the variable name
    // `*ptr_first_number`
    cout << "The value of ptr_first_number: " << *ptr_first_number << "\n";

    // first_number: 18
    // The value of ptr_first_number: 18

    //___________________________________________________________________________

    cout << "\n";

    // To print the memory address of a variable you add the ampersand symbol
    // `&` in front of the variable name.
    cout << "first_number is stored at the memory address " << &first_number
         << "\n";

    cout << "ptr_first_number points to the memory address " << ptr_first_number
         << "\n";

    // first_number is stored at the memory address 0x7ffeae6dc9a4
    // ptr_first_number points to the memory address 0x7ffeae6dc9a4

    //___________________________________________________________________________

    // If you change the value of of `first_number` then `ptr_first_number`
    // will also be updated because they point to the same location in
    // memory.
    first_number = 180;

    cout << "\n";

    cout << "first_number: " << first_number << "\n";
    cout << "The value of ptr_first_number: " << *ptr_first_number << "\n";

    // first_number: 180
    // The value of ptr_first_number: 180

    //___________________________________________________________________________

    // To update the value of a pointer, you have to dereference it first.
    // You do this by adding a `*` to the variable name.
    *ptr_first_number = 72;

    cout << "\n";
    cout << "first_number: " << first_number << "\n";
    cout << "The value of ptr_first_number: " << *ptr_first_number << "\n";

    // `first_number` and `ptr_first_number` are linked.
    // They point to the same location in memory. So updating the value of
    // the pointer will update the variable that it points to.

    // first_number: 72
    // The value of ptr_first_number: 72

    //___________________________________________________________________________
}
