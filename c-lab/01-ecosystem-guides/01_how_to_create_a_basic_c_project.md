# How to create a basic C program
_______________________________________________________________________________
## Requirements

You must have the following programs installed and available on your path.

1. make
2. gcc

_______________________________________________________________________________
### Create the project structure

Create the project directory
```sh
mkdir name-of-project
```
_______________________________________________________________________________

Enter the project directory
```sh
cd name-of-project
```
_______________________________________________________________________________

Create this project structure 
```sh
mkdir bin/
mkdir include/
mkdir src/
touch src/main.c
touch .clang-format
touch .clangd
touch .gitignore
touch Makefile
```
_______________________________________________________________________________
#### `src/main.c` 

Add this to the file
```c
int main() {}
```

This is the simplest possible C program you can create, 
that will compile successfully.

_______________________________________________________________________________
#### `.clang-format`

```yaml
Language: C
IndentWidth: 4
AccessModifierOffset: -4
ColumnLimit: 80
PointerAlignment: Left
AllowShortFunctionsOnASingleLine: true
```
_______________________________________________________________________________
#### `.clangd` 

Add this to the file
```yaml
CompileFlags:
  Add: [-std=c23, -Iinclude]
```
_______________________________________________________________________________
#### `.gitignore`

Add this to the file
```gitignore
/bin/
```
_______________________________________________________________________________
#### `Makefile`

Makefile
```make
.SILENT:
.PHONY: run clean
 
run:
	mkdir -p bin/
    gcc -std=c23 -Iinclude src/*.c -o bin/main
	./bin/main

clean:
	rm -rf bin/
```
_______________________________________________________________________________
