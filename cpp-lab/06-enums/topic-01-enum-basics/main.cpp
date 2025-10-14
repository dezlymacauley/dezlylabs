/*
    ABOUT: Enums

    enums is short for enumerated types

*/

#include <iostream>

/**

Available values:
- Red   : 0
- Green : 1
- Blue  : 2

*/
enum Color { 
    Red,
    Green,
    Blue
};

int main() {
    using namespace std;

    Color selected_color = Red;
    cout << "Selected color value: " << selected_color << endl;
    // Selected color value: 0
}
