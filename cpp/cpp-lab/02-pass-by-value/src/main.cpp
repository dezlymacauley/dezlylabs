#include <iostream>

using std::cout;

int main() {
    int first_number = 18;

    // This is pass by value.
    // num2 gets its own copy of the value of first_number.
    //
    // You can think of it as taking a picture of the value first_number,
    // and then giving that picture to second_number.
    //
    // So second_number is equal to the value of first_number,
    // only at this point in time.
    int second_number = first_number;

    cout << "first_number: " << first_number << "\n";
    cout << "second_number: " << second_number << "\n";
    // first_number: 18
    // second_number: 18

    //_________________________________________________________________________

    first_number = 50;

    cout << "\n";
    cout << "first_number: " << first_number << "\n";
    cout << "second_number: " << second_number << "\n";
    // first_number: 50
    // second_number: 18

    // When it comes to `pass by value`, the two variables are not connected.
    // So if the value of `first_number` changes,
    // the value of `second_number` will not be updated.

    // Pass by value is using your phone to take a picture of someone.
    // If that person changes their clothes after you took the picture,
    // the picture on your phone will not be updated.
    
    //_________________________________________________________________________
    // SECTION: Proof that first_number and second_number are stored at
    // different points in memory

    cout << "\n";

    cout << "first_number is stored at: " << &first_number << "\n";
    cout << "second_number is stored at: " << &second_number << "\n";

    // first_number is stored at: 0x7ffd85c2f0c4
    // second_number is stored at: 0x7ffd85c2f0c0

    //_________________________________________________________________________
}
