package main

import (
	"fmt"
	"topic-06-creating-a-user-defined-type/models"
)

func main() {
    var carOne *models.Car = models.NewCar("purple")
    fmt.Printf("%+v\n", carOne)
    // &{color:purple}

    // WARNING: The fmt line uses reflection

    // a special mechanism that can inspect the internal structure of any 
    // value at runtime — even private fields.
    // So make sure not to leave this in production code.

    carOne.Hoot();
    // Beep! Beep!

    fmt.Printf("The color of car one is %s\n", carOne.GetColor())
    // The color of car one is purple

    carOne.SetColor("orange")
    fmt.Printf("The color of car one is %s\n", carOne.GetColor())
    // The color of car one is orange
}
