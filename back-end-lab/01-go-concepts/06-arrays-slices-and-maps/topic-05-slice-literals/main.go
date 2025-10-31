package main

import "fmt"

func main() {

	// This is a slice literal
	// The syntax looks like a fixed-sized array except
	// that you don't specify the number of elements it should contain.
	// The compiler decides the length from the number of values provided.

	// Unlike a fixed-sized array, the slice does not directly hold values.
	// It is a small descriptor (pointer, length, capacity) that refers to an
	// underlying array where the actual values are stored.
	// In many cases that underlying array is allocated on the heap
	// (so it can live beyond the current stack frame),
	// but the Go compiler may sometimes keep it on the stack if it’s safe.

	// Go creates an underlying array, then makes a slice that points to it.
	prices := []float64{10.99, 8.99}
	fmt.Println(prices[0:1])
	// [10.99]

	// You can update the value of an index that already exists.
	prices[1] = 9.99
	fmt.Println(prices)
	// [10.99 9.99]

	// If you want to add a new value to the end of a dynamic array then you
	// have to do this:

	//_________________________________________________________________________

	// NOTE: This will create a new array that is not connected
	// to the original array

	updatedPrices := append(prices, 5.99)
	fmt.Println(updatedPrices)
	// [10.99 9.99 5.99]

	// The original array is the same.
	fmt.Println(prices)
	// [10.99 9.99]

	//_________________________________________________________________________

	// NOTE: If you want to add a value to the original array then you have
	// to do this:

    // The original array
	fmt.Println(prices)
    // [10.99 9.99]

	// This will create a new array and asign it to the variable prices
	prices = append(prices, 20.85)
	prices = append(prices, 40.73)
	prices = append(prices, 73.21)
	fmt.Println(prices)
	// [10.99 9.99 20.85 40.73 73.21]

	//_________________________________________________________________________

    // NOTE: How to add an element to the start of a slice

	prices = append([]float64{125.20}, prices...)
	fmt.Println(prices)
    // [125.2 10.99 9.99 20.85 40.73 73.21]

    //_________________________________________________________________________

}
