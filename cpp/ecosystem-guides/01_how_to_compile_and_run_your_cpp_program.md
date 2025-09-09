# How to compile and run your C++ program
_______________________________________________________________________________
## Requirements

You must have the following programs installed and available on your path.

1. make
2. clang

_______________________________________________________________________________
## Create a directory for your project and enter it

```sh
mkdir name-of-project
cd name-of-project
```
_______________________________________________________________________________
## Creat this project structure

```sh
mkdir src/
touch src/main.cpp
mkdir bin/
touch .gitignore
touch Makefile
```
_______________________________________________________________________________
### Add this to `src/main.cpp` 

This is the simplest possible C++ program you can create, 
that will compile successfully.

```cpp
int main() {}
```
_______________________________________________________________________________
### Add this to the `.gitignore` file

```gitignore
/bin/
```
_______________________________________________________________________________
### Add this to the `Makefile`

Makefile
```make
.SILENT:
.PHONY: run clean
 
run:
	mkdir -p bin/
	clang -x c++ src/main.cpp -o bin/main
	./bin/main

clean:
	rm -rf bin/
```
_______________________________________________________________________________
## Understanding the syntax of a Makefile

This is a safegaurd so to ensure that even if there are files in the project
called "run" or "clean", the Makefile will not confuse them for the commands
"run" and "clean" that are in the Makefile.

```
.PHONY: run clean
```

This tells the Makefile not to show the actual commands that it is running,
but instead to only show the output of those commands in the terminal. 
```
.SILENT:
```

This command `make run` will do create a directory called `bin/` if it does
not exist.
```
run:
    mkdir -p bin/
    clang -x c++ src/main.cpp -o bin/main
    ./bin/main
```

`clang -x c++ src/main.cpp -o bin/main` means that clang will be used 
to compile `src/main.cpp` into and executable binary that will be saved in
`bin/main`

The `-x` in `clang -x c++` means explict language mode. Clang can also be
use to compile C code. 

So this setting makes it explict that you are compiling a C++ file.

_______________________________________________________________________________
## Run these commands from the root directory of you project

_______________________________________________________________________________

To run you project use this command: 

`make run`
_______________________________________________________________________________

If `run` is the first command in your Makefile, then you can use the command:
`make`

_______________________________________________________________________________
