/*

ABOUT: Dynamic-sized arrays

In Go a dynamic-sized array is called a `slice`.
The equivalent of a slice in Rust would be a Vector.

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
    // SECTION: Dynamic-Sized Arrays (aka a Slice)

    // This will create a dynamic-sized array, from a fixed-sized array.
    // This will get all the values from 
    // index 1 to index 3 (excluding index 3):
    // 9.99, 45.99 

    featuredPrices := prices[1:3]
    fmt.Println(featuredPrices)
    // [9.99 45.99]

    //_________________________________________________________________________
    // SECTION: Creating a dynamic-Sized Arrays (aka a Slice)


    //_________________________________________________________________________

}
