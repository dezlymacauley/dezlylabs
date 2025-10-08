/*

ABOUT: `package main` and `func main()`

The Go compiler expects every file to start with a package declaration.
A package declaration tells the Go compiler
what package the file belongs to.
So this file `main.go` is part of the package called `main`

When it comes to packages, the name `main` is special in Go.
In Go `package main` tells the Go compiler that this file is part of
an executable program (and not a re-usable library of code).

Once the Go compiler has identified package main, 
it then looks for `func` main.

`func main()` is the entry point of your Go program.
In simple terms, it tells Go, please start here.

NOTE: Multiple files can belong to `package main`. 
However, a Go program must have exactly one `func main()`, 
because an executable can only have one entry point.

*/

package main

// This is an import declaration
import (
    // This imports the `fmt` package from the Go standard library.
    // This package includes built-in functions like `Println`
    // which allow you to print messages to the terminal
	"fmt"
)

func main() {
    fmt.Println("Hello world")

    // The function `greetSuperman` is not in this `main.go` file.
    // The `greetSuperman` function is in file `superhero_greetings.go`
    // However, because `superhero_greetings.go` is part of `package main`,
    // you can use it as if it was written in this file.
    greetSuperman()
}
