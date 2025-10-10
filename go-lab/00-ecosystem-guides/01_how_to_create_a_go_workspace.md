# How to create a Go workspace
_______________________________________________________________________________

Create a directory for the workspace
```sh
mkdir name-of-workspace
```
_______________________________________________________________________________

Intialize the workspace
```sh
cd name-of-workspace
```

Run all of the following commands while in this directory.
_______________________________________________________________________________

Intialize the workspace
```sh
go work init
```
_______________________________________________________________________________

### How to add projects to the workspace

```sh
mkdir project-alpha
go mod init project-alpha
```

Intialize the module

```sh
(cd project-alpha && go mod init go-lab/project-alpha)
```

Note the `()` is the syntax for a sub shell in Bash. 

It allows you to temporarily change to a directory and run commands 
from that directory.

This will create a `go.mod` file inside `go-lab/project-alpha/`

go-lab/project-alpha/go.mod
```gomod
module go-lab/project-alpha

go 1.25.1
```

_______________________________________________________________________________

Add a main.go file to the module

```sh
touch project-alpha/main.go
```

Add this to the file:
```sh
cat > project-alpha/main.go << 'EOF'
package main

import (
	"fmt"
)

func main() {
	fmt.Println("Hello from project-alpha")
}
EOF
```

_______________________________________________________________________________
### Add the project to the workspace

```sh
go work use project-alpha 
```

The project will now be added to your workspace.

You can see it in the `go.work` file
```gowork
go 1.25.1

use ./project-alpha
```
_______________________________________________________________________________
### How to run a specific project

```sh
go run ./name-of-project
```

Or 

```sh
cd name-of-project
go run .
```

_______________________________________________________________________________
