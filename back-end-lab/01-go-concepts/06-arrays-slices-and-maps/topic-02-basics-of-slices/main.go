/*

ABOUT: Slice

A slice is simply a reference to a section of values in an an existing array.

*/

package main

import "fmt"

func main() {
    //_________________________________________________________________________
    // SECTION: Fixed-Sized Arrays	

    productNames := [4]string{"A Book"}

    // index              0      1      2      3    
	prices := [4]float64{10.99, 9.99, 45.99, 20.0}
	fmt.Println(prices)
	// [10.99 9.99 45.99 20]

	productNames[2] = "A Carpet"
	fmt.Println(productNames)
	// [A Book  A Carpet ]

	fmt.Println(prices[2])
	// 45.99

    //_________________________________________________________________________
    // SECTION: Slices

    // In Go a slice is a descriptor or view into and existing array.

    // This will create a dynamic-sized array, from a fixed-sized array.
    // This will get all the values from 
    // index 1 to index 3 (excluding index 3):
    // 9.99, 45.99 

    featuredPrices := prices[1:3]
    fmt.Println(featuredPrices)
    // [9.99 45.99]

    //_________________________________________________________________________
    // SECTION: Another example of a slice

    // A fixed-sized array that contains five 64 bit floating points 

    // Indexes:                 0      1      2      3      4
    fightScores := [5]float64{20.23, 49.12, 19.23, 23.40, 39.56}
    
    fmt.Println(fightScores)
    // [20.23 49.12 19.23 23.4 39.56]

    // This will be a list of reference for the the first three values in
    // the slice.
    // 0:3 means index 0 to 3 (excluding 3)
    // So that means index 0, index 1, and index 2
    firstThreeScores := fightScores[0:3]

    fmt.Println(firstThreeScores)
    // [20.23 49.12 19.23]

    // NOTE: A slice in Go does not own its data.

    // A slice is a struct and without complicating things,
    // all you need to know is that one of the fields contains 
    // a pointer to the starting element (the index in the original array
    // that you want to start from.)
    // to the values at the indexes of the original arrays.
    // Also not that since variables in Go are mutable by default,
    // this means that these are mutable references.

    //_________________________________________________________________________

    // NOTE: If you change values in the original array,
    // the values in the slice will be affected if the slice is based 
    // on the section changed in the original array. 

    // I am changing the first two scores in the slice.
    fightScores[0] = 435.20
    fightScores[1] = 240.50

    fmt.Println(fightScores)
    // [435.2 240.5 19.23 23.4 39.56]

    fmt.Println(firstThreeScores)
    // [435.2 240.5 19.23]
    
    // The values in the slice have changes

    //_________________________________________________________________________

    // NOTE: If you change values in the slice,
    // the values in the original will be affected if those values are based 
    // on the section changed in the slice. 

    // I am changing the first two scores in the slice
    firstThreeScores[0] = 10.50
    firstThreeScores[1] = 83.25

    fmt.Println(firstThreeScores)
    //[10.5 83.25 19.23]

    fmt.Println(fightScores)
    // [10.5 83.25 19.23 23.4 39.56]

    // As you can see, the original array has been affected.

    //_________________________________________________________________________

}
