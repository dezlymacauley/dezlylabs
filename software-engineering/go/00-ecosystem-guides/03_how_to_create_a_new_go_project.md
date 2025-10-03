# How to create a new Go project
_______________________________________________________________________________
### Create a directory for your project
```sh
mkdir name-of-project
```
_______________________________________________________________________________
### Enter the directory
```sh
cd name-of-project
```

**Note:**
- For the rest of this guide, please run these commands from this directory.

_______________________________________________________________________________
### Create a `go.mod` file

This is a file that will intialize the project as a Go module.

This tells Go that this directory is a Go program 
that can have its own dependencies.

The convention in Go is to use the GitHub url where you intend to save
your project. 

It's fine if you have not created this repo on GitHub yet.
```sh
go mod init github.com/github-account-name/name-of-project
```

**Note:**
- If you are just performing a quick experiement then you can use this:

```sh
go mod name-of-project
```
_______________________________________________________________________________
### Create a .gitignore file

```sh
touch .gitignore
```
_______________________________________________________________________________
### Create a `main.go` file

```sh
touch main.go
```

Run this command to setup the `main.go` file:
```sh
cat > main.go << 'EOF'
package main

func main() {}
EOF
```

This is the absolute minimum required to compile a Go program.
_______________________________________________________________________________
### Run your program

If it is a single file program then you can just do this.
```sh
go run main.go
```

For most go programs, you'll want to do this instead:
```sh
go run .
```
_______________________________________________________________________________
