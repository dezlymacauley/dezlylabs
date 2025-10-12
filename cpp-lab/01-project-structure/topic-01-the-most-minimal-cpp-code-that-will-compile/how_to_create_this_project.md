# The most minimal C++ program that will compile 
_______________________________________________________________________________

```sh
mkdir name-of-project
cd name-of-project
```

```sh
mkdir bin/
touch main.cpp
touch .gitignore
```

Add this to the `.gitignore` file
```sh
bin/
```

Add this to the `main.cpp` file
```cpp
int main() {}
```

Note that in modern C++, the `main` function will implicity 
return an int (integer).

If the program reached the end of the `main` function it will return 0,
otherwise it will return 
_______________________________________________________________________________
## To run this file:

The syntax is
```
clang++ name_of_cpp_file.cpp \
-o location/name_of_executable_binary_file
```

```sh
clang++ main.cpp \
-o bin/main
```

```sh
./bin/main
```
_______________________________________________________________________________
