package main

import "fmt"

func main() {
    prices := []float64{10, 20, 30}
    
    // 0%, 7%, 10%, 15%
    taxRates := []float64{0, 0.7, 0.1, 0.15}

    // Each key will be one of the `taxRates`
    // The values will be the calculated values for each prices

    // E.g.
    // Key : Value
    // 0 : 10 + (0% tax), 20 + (0% tax), 30 + (0% tax)
    // 0.7 : 10 + (7% tax), 20 + (7% tax), 30 + (7% tax)
    // 0.1 : 10 + (1% tax), 20 + (1% tax), 30 + (1% tax)
    // 0.15 : 10 + (15% tax), 20 + (15% tax), 30 + (15% tax)

    // This is how you create an empty map in Go
    result := make(map[float64][]float64)

    // This is a for loop to Go through each tax rate
    for _, taxRate := range taxRates {

        taxIncludedPrices := make([]float64, len(prices))

        // For each tax rate, a tax included price must 
        // be calculated for each price in the `prices` list.
        for priceIndex, price := range prices {
            taxIncludedPrices[priceIndex] = price * (1 + taxRate)
        }

        result[taxRate] = taxIncludedPrices

    }

    fmt.Println(result)
    // map[0:[10 20 30] 0.1:[11 22 33] 0.15:[11.5 23 34.5] 0.7:[17 34 51]]

}
