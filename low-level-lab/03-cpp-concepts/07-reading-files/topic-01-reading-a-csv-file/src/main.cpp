#include <fstream>
using std::ifstream;

#include <print>
using std::println;

int main() {

    //_________________________________________________________________________
    
    // SECTION: Step 1 - Attempt to open the file
    // `ifstream` means `input file stream`

    // This sets up a stream (a flow of data), from a file, 
    // into your C++ program.

    // This line does not read from the file, 
    // it simply attempts to open a file that you specify so that you can
    // read from it.
    ifstream userDataFile("user_data.csv");

    //_________________________________________________________________________
   
    // SECTION: Step 2 - Error handling

    // Check if the file was successfully opened befor proceeding with the
    // rest of the program.
    if (userDataFile.is_open() == false) {
        println("Error: user_data.csv could not be opened.");
        return 1;
    }
        
    println("Success: user_data.csv was opened.");

    //_________________________________________________________________________
    // SECTION: Step X - Close the file 

    userDataFile.close();

}
