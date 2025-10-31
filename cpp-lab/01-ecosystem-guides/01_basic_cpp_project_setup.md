# Basic C++ Project Setup
_______________________________________________________________________________
### How to setup C++ on Arch Linux

Install the clang package
```sh
sudo pacman --needed clang
```

This package includes the following:
1. `clang++` A compiler for turning C++ code into machine code.

The compilation settings can be set by usin a `Makefile` in your project.

2. `clangd` Language support for C++ in Neovim. 

It can be configured by using a `.clangd` file in your project.

3. `clang-format` Formats your C++ code.

It can be configured by adding a `.clang-format` file to your project.
_______________________________________________________________________________
### How to setup the project

Create the project directory
```sh
mkdir basic-cpp-project
```
_______________________________________________________________________________

Enter the project directory
```sh
cd basic-cpp-project
```
_______________________________________________________________________________

Create the project structure
```sh
mkdir bin/
mkdir src/
mkdir include/
touch src/main.cpp
touch .clang-format
touch .clangd
touch .gitignore
touch Makefile
```
_______________________________________________________________________________
### Configure the project
_______________________________________________________________________________
#### `src/main.cpp`

```cpp
int main() {}
```

Add this to the `main.cpp` file

Note that in modern C++, the `main` function will implicity 
return an int (integer).

If the program reached the end of the `main` function it will return 0,
otherwise it will return 

If you really want to be explicit

```cpp
int main() {
   return 0;
}
```


_______________________________________________________________________________
#### `.clang-format`

```yaml
Language: Cpp
IndentWidth: 4
AccessModifierOffset: -4
ColumnLimit: 80
PointerAlignment: Left
AllowShortFunctionsOnASingleLine: true
```
_______________________________________________________________________________
#### `.clangd`

```yaml
CompileFlags:
  Add: [-std=c++23, -Iinclude]
```
_______________________________________________________________________________
#### `.gitignore`

```gitignore
bin/
```
_______________________________________________________________________________
#### `Makefile`

```make
.SILENT:
.PHONY: run clean

run:
	mkdir -p bin/
	clang++ -std=c++23 -Iinclude src/*.cpp -o bin/main
	./bin/main

clean:
	rm -rf bin/
```
_______________________________________________________________________________
_______________________________________________________________________________
## How to run this file

Make sure that you are in the root of the project and then run this command:

```sh
make
```
_______________________________________________________________________________
