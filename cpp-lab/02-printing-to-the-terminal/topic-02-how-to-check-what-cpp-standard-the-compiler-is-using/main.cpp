#include <cstdio>

int main() {
    // `__cplusplus` is a macro (C++ code that generates C++ code)
    // that returns a `long int`
    // %ld is the format specifier for `long int`
    printf("Compiled using the C++ standard: %ld\n", __cplusplus);
    // Compiled using the C++ standard: 201703

    // The format is: YYYYMM (Year, Month)
    // 201703 mean 2017, March
    // So this code was compiled using the C++17 version.

    // So that means that when you run:
    // clang++ main -o bin/main
    //
    // Without specifying a version, C++17 is used.

    /*
        To compile your code with other standards edit the Makefile:

        clang++ -std=c++11 main.cpp -o main   # Compile with C++11
        clang++ -std=c++14 main.cpp -o main   # Compile with C++14
        clang++ -std=c++20 main.cpp -o main   # Compile with C++20
        clang++ -std=c++23 main.cpp -o main   # Compile with C++23
    */
}
