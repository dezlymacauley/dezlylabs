# How to create a basic Go project
_______________________________________________________________________________

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

Create a `go.mod` file
```sh
go mod init github.com/github-account-name/name-of-project
```

This is a file that will intialize the project as a Go module.

This tells Go that this directory is a Go program 
that can have its own dependencies.

The convention in Go is to use the GitHub url where you intend to save
your project. It's fine if you have not created this repo on GitHub yet,
or don't have a GitHub account created yet.

_______________________________________________________________________________
### Create the project structure

```sh
touch .gitignore
touch Makefile
touch main.go
```
_______________________________________________________________________________

#### `.gitignore`

```gitignore
# Executable binaries
/bin/
```
_______________________________________________________________________________

#### `Makefile`

```make
.SILENT:
.PHONY: run build runbin 

run:
	go run .

build:
	go build -o bin/main .

runbin:
	./bin/main

clean:
	rm -rf bin/
```
_______________________________________________________________________________

#### `main.go`

Add this to the file:
```go
package main

func main() {}
```

This is the absolute minimum required to compile a Go program.
_______________________________________________________________________________
### Run your program

To run your program use this command:
```sh
go run .
```
_______________________________________________________________________________
