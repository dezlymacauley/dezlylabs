package main

import "fmt"

func main() {

    // This creates a slice (a list) of 8 bit unsigned integers.
    // This slice will have an initial length of 3, 
    // so it can store three values. 

    // Each of the three values will be set to the zero value of an uint8,
    // which is 0.

    // However this slice is not limited to only 3 values, 
    // you can add more later. It's just in Go, you always have to set
    // an initial length when you use the make syntax to create a slice.
    // The initial length can be set to 0 if you want.

    comboHits := make([]uint8, 3)
    fmt.Println("comboHits is", comboHits)
    // comboHits is [0 0 0]

    comboHits[0] = 20
    comboHits[1] = 40
    comboHits[2] = 50
    fmt.Println("comboHits is", comboHits)
    // comboHits is [20 40 50]

    //_________________________________________________________________________
   
    // NOTE: This will fail because you are trying to add 
    // more than the initial length of 3 (last value is index 2)

    // comboHits[3] = 90
    // fmt.Println("comboHits is", comboHits)
    
    //_________________________________________________________________________
    // To add more than the initial length use the `append` keyword.

    comboHits = append(comboHits, 150)
    comboHits = append(comboHits, 26)
    comboHits = append(comboHits, 200)
    fmt.Println("comboHits is", comboHits)
    // comboHits is [20 40 50 150 26 200]

    //_________________________________________________________________________


}
