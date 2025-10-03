package main

import "fmt"

func main() {
    // This is how to declare and view the contents of an array
    prices := [3]float64{25.8, 10.23, 35.9}
    fmt.Println(prices)
    // [25.8 10.23 35.9]

    // How to access a specific element
    fmt.Println("The first price is", prices[0])
    // The first price is 25.8

    productNames := [4]string{"Jelly Beans"}
    fmt.Println(productNames)
    // [Jelly Beans   ]


    productNames[3] = "Coco Muffin"
    fmt.Println(productNames[3])
    // Coco Muffin
}
