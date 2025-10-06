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

}
