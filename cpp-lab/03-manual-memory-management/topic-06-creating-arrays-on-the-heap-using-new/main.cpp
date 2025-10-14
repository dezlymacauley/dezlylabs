/*

ABOUT: How to create dynamic arrays in C++ using new
This is the classic C++98

*/

#include <cstring>
#include <iostream>

using std::cout, std::endl;

int main() {
    //_________________________________________________________________________
    // SECTION: Example 1 

    // This will create a dynamic array of 5 integers,
    // and each integer will be given the initial value of 0
    int* listOfNumbers = new int[5]{12, 30, 50, 5, 17};

    for (int i = 0; i < 5; i++) {
        cout << listOfNumbers[i] << " ";
    }

    cout << endl;
    // 12 30 50 5 17 

    // NOTE: Don't forget the clean up
    delete[] listOfNumbers;
    listOfNumbers = nullptr;  

    //_________________________________________________________________________
    // SECTION: Example 2
  
    char* listOfLetters = new char[4];


    // Initialize each element in the array
    listOfLetters[0] = 'C';
    listOfLetters[1] = 'a';
    listOfLetters[2] = 't';
    
    // NOTE: Don't forget that the last element is the Null terminator 
    // (marks the end of a C-string)
    listOfLetters[3] = '\0'; 

    for (int i = 0; i < 4; i++) {
        cout << listOfLetters[i];
    }
    cout << endl;
    // Cat


    // Another way to initialize a dyanamic array of characters is to
    // use strncpy. Please ensure that the number of characters + the null
    // terminator does not exceed the length of the character array.
    // E.g. Bat + nullptr = 4 elements which is fine because the array
    // was declared as:
    // char* listOfLetters = new char[4];

    // The syntax for strncpy is: array, 
    // the array where the string should go, 
    // "a string containing the elements you want to add", 
    // number of elements. This should be the number of characters + 1

    // This will overwrite the contents of the string.
    strncpy(listOfLetters, "Bat", 4);
    
    for (int i = 0; i < 4; i++) {
        cout << listOfLetters[i];
    }
    cout << endl;
    // Bat

    // NOTE: Don't forget the clean up
    delete[] listOfLetters;
    listOfLetters = nullptr;  

    //_________________________________________________________________________

}
