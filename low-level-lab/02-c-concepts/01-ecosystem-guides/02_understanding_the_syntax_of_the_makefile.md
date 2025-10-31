# Understanding the syntax of a Makefile
_______________________________________________________________________________

Here is a simple Makefile for a C project
```make
.SILENT:
.PHONY: run clean


# To use this command just open the terminal in the root directory of this
# project and run the command:
# `make run`

# Also note that since this is the first command in this Makefile,
# you can just use the command `make`, and it will do the same thing as
# `make run`

# 1. This `make run` command will do create a directory called `bin/` 
# if it does not exist.

# 2. Then it will use `clang++` (a compiler for C++), to compile your
# code using the C++ 2023 standard.
# `-Iinclude` tells the compiler to look for header files in a directory
# in your project called `include`. 
# `src/*.cpp` -o bin/main tells the compiler to compile all C++ files inside
# of the `src` directory and create an executable binary called `main` inside
# the `bin` directory.
run:
	mkdir -p bin/
	clang++ -std=c++23 -Iinclude src/*.cpp -o bin/main
	./bin/main

clean:
	rm -rf bin/
```
_______________________________________________________________________________

This will silence the default output from the `make` command,
which is to print out the intruction in the Makefile that it is executing

```sh
.SILENT:
```
_______________________________________________________________________________

This is a safegaurd so to ensure that even if there are files in the project
called "run" or "clean", the Makefile will not confuse them for the commands
"run" and "clean" that are in the Makefile.

```make
.PHONY: run clean
```
_______________________________________________________________________________

To use a command in a Makefile all you need to run `make name_of_command`.

Tip: If you just run `make`,  it will automatically execute 
the first command in a Makefile.
_______________________________________________________________________________

```make
run:
    mkdir -p bin/
    gcc -std=c23 -Iinclude src/*.c -o bin/main
    ./bin/main
```

This command will create a directory called `bin` if it doesn't exist.

Then it will use `gcc`, which is a C compiler to compile all of the files
inside the `src` directory that that end with `.c`, and will create an
executable binary taht will be saved in `bin/main`

`-std=c23` will compile your code using the C 2023 standard

`-I` tells the compiler where to look 
for header files (Files that end with .h).

`Iinclude` means look for header files inside 
of the directory called `include`.

_______________________________________________________________________________

To run this command, run `make clean`. 

This will remove the `bin` directory to save disk space 
on your system when your are not using the project.

```sh
clean:
	rm -rf bin/
```
_______________________________________________________________________________
### Run these commands from the root directory of you project

To run you project use this command: 

`make run`
_______________________________________________________________________________

If `run` is the first command in your Makefile, then you can use the command:
`make`

_______________________________________________________________________________

To delete the `bin/` directory to save space on your system, 
you can use this command:

```sh
make clean
```
_______________________________________________________________________________
