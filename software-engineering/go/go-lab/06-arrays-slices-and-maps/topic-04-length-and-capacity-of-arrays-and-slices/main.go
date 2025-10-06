package main

import "fmt"

func main() {

	prices := [4]float64{10.99, 9.99, 45.99, 20.0}
	fmt.Println(prices)
	// [10.99 9.99 45.99 20]

	//_________________________________________________________________________
	// SECTION: Example 1 - Leaving out the starting index

	// If you omit the starting index, the slice will start from index 0
	// So :3 means index0 to index 3 (excluding 3)
	firstThreePrices := prices[:3]
	fmt.Println(firstThreePrices)
	// [10.99 9.99 45.99]

	//_________________________________________________________________________
	// SECTION: Example 2 - Leaving out the ending index

	allPricesExceptTheFirstPrice := prices[1:]
	fmt.Println(allPricesExceptTheFirstPrice)
	// [9.99 45.99 20]

	//_________________________________________________________________________
	// SECTION: len = length = The number of items in an array or slice
	// currently contains

	fmt.Println(len(prices))                       //4
	fmt.Println(len(firstThreePrices))             // 3
	fmt.Println(len(allPricesExceptTheFirstPrice)) // 3

	//_________________________________________________________________________
	// SECTION: cap = capacity
	// Capacity = (items the slice contains)
	// + (remaining items in the original array after the slice ends)

	//_________________________________________________________________________

	fmt.Println(cap(prices))
	// For a full array (or slice covering the whole array),
	// capacity equals length because there's nothing left to expand into.
	// Items it contains: 4 (indices 0, 1, 2, 3)
	// Items remaining: 0
	// Capacity: 4 + 0 = 4

	/*
	        // Here's the original fixed-sized array

	        // Indexes:            0      1     2      3
			prices := [4]float64{10.99, 9.99, 45.99, 20.0}
	        //[10.99 9.99 45.99 20]
	*/

	//_________________________________________________________________________

	fmt.Println(cap(firstThreePrices)) // 4

	/*
			firstThreePrices := prices[:3]
			fmt.Println(firstThreePrices)
			// [10.99 9.99 45.99]

	        [:3] means index 0 to index 3 (excluding 3)

	        Items slice contains: 3 (indices 0, 1, 2)
	        Items not covered in the original array: 1
	        index 3 from the original array is not covered.

	        Capacity: 3 + 1 = 4
	*/

	//_________________________________________________________________________
	fmt.Println(cap(allPricesExceptTheFirstPrice)) // 3

	/*

	   // allPricesExceptTheFirstPrice := prices[1:]
	   // fmt.Println(allPricesExceptTheFirstPrice)
	   // // [9.99 45.99 20]

	   [1:] means index 1 to the end of the last index (including the last index)

	   Items slice contains: 3 (indices 1, 2, 3)
	   Items not covered from the original array: 0 (None)
	   index 3 is the last index in the original array

	   Capacity: 3 + 0 = 3

	*/

	//_________________________________________________________________________

}
