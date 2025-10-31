`go get`	Add a library dependency to your project	Project/module
`go install`	Install a CLI tool globally	Global/bin
`go mod tidy`	Clean up go.mod and go.sum	Project/module

_______________________________________________________________________________

Go to `pkg.go.dev` and search for the package you want to use

E.g. I want to use `gin`

There should be a link that copies the Github url


Add it to your code inside the import block.
```go
package main

import (
    // A web framework written in Go
    "github.com/gin-gonic/gin"
)

func main() {

}
```

Then run `go mod tidy`

_______________________________________________________________________________
