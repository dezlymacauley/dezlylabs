/*

ABOUT: How to create dynamic arrays in C++ using vector
This is the classic C++98

*/

#include <cstring>
#include <iostream>
#include <vector>

using std::cout, std::endl;
using std::vector;

int main() {
   // Using vector to store 4 characters (3 characters + the null terminator)
    vector<char> listOfLetters(4);

    // Initialize each element in the array
    listOfLetters[0] = 'C';
    listOfLetters[1] = 'a';
    listOfLetters[2] = 't';
    
    // NOTE: Don't forget that the last element is the Null terminator 
    // (marks the end of a C-string)
    listOfLetters[3] = '\0'; 

    // To print the contents of the array, just do this:
    cout << listOfLetters.data() << endl;
    // Cat

    // How to update a array of characters using strncpy
    // The syntax is: array.data, "The contents", array.size()"

    // NOTE: Always check that the number of elements in the contents +
    // 1 can fit array.size()
    strncpy(listOfLetters.data(), "Bat", listOfLetters.size());
    cout << listOfLetters.data() << endl;
    // Bat

    //_________________________________________________________________________

}
