# `.SILENT` This will silence the default output from Make
# which is to print out the intruction in the Makefile that it is executing
.SILENT:

# `.PHONY` This is a safegaurd so to ensure that even if there are files 
# in the project called "run" or "clean", 
# the Makefile will not confuse them for the commands "run" and "clean" 
# that are in the Makefile.
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
