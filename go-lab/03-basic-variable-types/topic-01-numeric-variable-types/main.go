package main

import "fmt"

func main() {
	//_________________________________________________________________________
	// SECTION: Signed Integers (Stores number that are positive or negative)

	// int8, int16, int32, int64
    // `rune` can aslo be used an alias for int32
    // Go does not have a `char` type like C, so you would use a `rune` to
    // represent a unicode character. 

	var accountBalance int32 = -520
	fmt.Println(accountBalance)
	fmt.Println("Account Balance is", accountBalance)
	fmt.Printf("account Balance is %d\n", accountBalance)

	//_________________________________________________________________________
	// SECTION: Unsigned Integers (Can store only positive numbers)

	// uint8, uint16, uint32, uint64
	// `byte` can also be used as an alias for uint8

	var age uint8 = 18
	fmt.Println(age)
	fmt.Println("Age is", age)
	fmt.Printf("Age is %d\n", age)

	//_________________________________________________________________________
	// SECTION: Floating Point Numbers (Numbers with a decimal part)

	// 32bits
	var sale_price float32 = 3.50
	fmt.Println(sale_price)
	fmt.Println("Sale price is", sale_price)
	fmt.Printf("Sale price is %.2f\n", sale_price)

    // 64bits
    var cost_price float64 = 73.3929232
	fmt.Println(cost_price)
	fmt.Println("Cost price is", cost_price)
	fmt.Printf("Cost price is %.2f\n", cost_price)

	//_________________________________________________________________________
	// SECTION: Letting Rust infer the type

    // This is the same as `var total int = 27`
    // What is the size of this int? 
    // Depending on the operating system, it will be 32bits or 64bits 
    total := 27
    println(total)

	//_________________________________________________________________________
}
