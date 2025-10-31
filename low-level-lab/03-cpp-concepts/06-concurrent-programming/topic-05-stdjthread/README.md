# Basic C++ Project Setup
_______________________________________________________________________________
## How to setup C++ on Arch Linux

Install the clang package
```sh
sudo pacman --needed clang
```

This package includes the following:
1. `clang++` A compiler for turning C++ code into machine code.
2. `clangd` Language support for C++ in Neovim.
3. `clang-format` Formats your C++ code based on the `.clang-format` file
in your project.
_______________________________________________________________________________
## How to create a new C++ project

```sh
mkdir name-of-project
cd name-of-project
```

```sh
mkdir bin/
mkdir src/
touch src/main.cpp
touch .clang-format
touch .clangd
touch .gitignore
touch Makefile
```
_______________________________________________________________________________
## Here's what to add to each file:
_______________________________________________________________________________
#### `src/main.cpp`

```cpp
int main() {
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
AllowShortFunctionsOnASingleLine: false
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


Add this to the `.gitignore` file
```sh
bin/
```

Add this to the `main.cpp` file

Note that in modern C++, the `main` function will implicity 
return an int (integer).

If the program reached the end of the `main` function it will return 0,
otherwise it will return 
_______________________________________________________________________________
## How to run this file

Make sure that you are in the root of the project and then run this command:

```sh
make
```
_______________________________________________________________________________
