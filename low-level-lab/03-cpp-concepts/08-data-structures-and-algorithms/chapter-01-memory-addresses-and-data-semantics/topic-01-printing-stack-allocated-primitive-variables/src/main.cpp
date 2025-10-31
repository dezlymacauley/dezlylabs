/* 

ABOUT: Stack-Allocated Primitive Variables

*/

#include <print>

using std::println;

int main() {

    //_________________________________________________________________________
    // SECTION: Signed Integer Types

    short short_variable = 10;
    println("short_variable is {}", short_variable);
    // short_variable is 10

    int int_variable = 5;
    println("int_variable is {}", int_variable);
    // int_variable is 5

    long long_variable = 1000;
    println("long_variable is {}", long_variable);
    // long_variable is 1000

    long long long_long_variable = 1000000;
    println("long_long_variable is {}", long_long_variable);
    // long_long_variable is 1000000

    //_________________________________________________________________________
    // SECTION: Unsigned Integer Types 

    unsigned short unsigned_short_variable = 8;
    println("unsigned_short_variable is {}", unsigned_short_variable);
    // unsigned_short_variable is 8

    unsigned int unsigned_int_variable = 20;
    println("unsigned_int_variable is {}", unsigned_int_variable);
    // unsigned_int_variable is 20


    unsigned long unsigned_long_variable = 8000;
    println("unsigned_long_variable is {}", unsigned_long_variable);
    // unsigned_long_variable is 8000

    unsigned long long unsigned_long_long_variable = 9000000;
    println("unsigned_long_long_variable is {}", unsigned_long_long_variable);
    // unsigned_long_variable is 1000000

    //_________________________________________________________________________
    // SECTION: Floating Point Types

    float float_variable = 3.14f;
    println("float_variable is {}", float_variable);
    // float_variable is 3.14

    double double_variable = 2.718;
    println("double_variable is {}", double_variable);
    // double_variable is 2.718

    long double long_double_variable = 1.618;
    println("long_double_variable is {}", long_double_variable);
    // long_double_variable is 1.6180000000000001048

    //_________________________________________________________________________

    // SECTION: Boolean

    bool isTeamLeader = true;
    println("isTeamLeader is {}", isTeamLeader);
    // isTeamLeader is true

    bool hasPremiumMembership = false;
    println("hasPremiumMembership is {}", hasPremiumMembership);
    // hasPremiumMembership is false

    //_________________________________________________________________________
    // SECTION: Character

    char char_variable = 'A';
    println("char_variable is {}", char_variable);
    // char_variable is A

    //_________________________________________________________________________

}
