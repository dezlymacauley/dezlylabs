#include <iostream>

// In C++ a function must be defined before it can be used,
// unless you are using a function prototype.

// This is comment below the function is a document comment.
// When someone hovers over the function, they will recieve some helpful
// information on how to use it.

/**
* Returns the sum of two integer values.
* `num_1` and `num_2` are passed in as copies of the original values.
* These numbers are added and returned by the function.
*/
int Add(int num_1, int num_2) {
    int total = num_1 + num_2;
    return total;
}

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
