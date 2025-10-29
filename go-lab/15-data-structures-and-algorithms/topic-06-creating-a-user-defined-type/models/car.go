package models

// NOTE: In Go lowercase means private, and uppercase means public.

// So the struct `Car` is public (which means that it can be accessed
// by code that imports this file), however the field `color` is private.

// So color can only be accessed be code that is part of the package `models`

type Car struct {
	color string
}

// How to create a constructor function in Go
func NewCar(color string) *Car {
	return &Car{
		color: color,
	}
}

// Associated functions (aka methods)

// regular method
func (c Car) Hoot() {
    println("Beep! Beep!")
}


// How to create getter function
func (c Car) GetColor() string {
    return c.color
}

// How to create a setter function
// WARNING: For setter functions, use *NameOfStruct
func (c *Car) SetColor(newColor string) {
    c.color = newColor
}
