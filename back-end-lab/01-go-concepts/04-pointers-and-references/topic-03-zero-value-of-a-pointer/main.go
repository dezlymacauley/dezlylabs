package main

import "fmt"

func isThisPointerNil(pointer *int8) bool {
   if pointer == nil {
        return true
   } else {
        return false
   }
}

func main() {
	var firstNum *int8
    fmt.Printf("Is firstNum nil: %t\n", isThisPointerNil(firstNum))
    // Is firstNum nil: true
}
