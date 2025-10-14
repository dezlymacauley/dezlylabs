#include <iostream>

// In C++ if you want to used a function before it is defined,
// you must have a `function declaration` (also called the function prototype)
int Add(int num_1, int num_2);

int main() {
    using std::cout, std::cin, std::endl;

    int x{0};
    int y{0};

    cout << "Enter a value for x: ";
    cin >> x;

    cout << "Enter a value for y: ";
    cin >> y;

    cout << endl;

    int result = Add(x, y);
    cout << "The sum of " << x << " and " << y << " is " << result << endl;

    return 0;
}

/**
* Returns the sum of two integer values.
* `num_1` and `num_2` are passed in as copies of the original values.
* These numbers are added and returned by the function.
*/
int Add(int num_1, int num_2) {
    int total = num_1 + num_2;
    return total;
}
