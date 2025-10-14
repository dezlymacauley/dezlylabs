/*

ABOUT: How to create dynamic arrays in C++ using vector
This is 

*/

#include <cstring>
#include <iostream>
#include <vector>

using std::cout, std::endl;
using std::vector;

int main() {
    // Create a vector of 5 integers and initialize it
    vector<int> listOfNumbers = {12, 30, 50, 5, 17};

    for (int i = 0; i < listOfNumbers.size(); i++) {
        cout << listOfNumbers[i] << " ";
    }

    cout << endl;
    // 12 30 50 5 17 

    // NOTE: One of the advantages of `vector` is that you do not need
    // the clean up step.
   
    // vector<int> is a not a pointer but a stack-allocated object.
    // This means the variable lives on the stack but the actual elements
    // are stored on the heap.
   
    // When the vector goes out of scope, C++ will automatically call the
    // destructor of the object when it goes out of scope.

    //_________________________________________________________________________

}
