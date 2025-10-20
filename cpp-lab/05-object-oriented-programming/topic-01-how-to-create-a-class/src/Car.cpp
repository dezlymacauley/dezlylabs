/*
    ABOUT: This file is where you add the implementation for the functions
    that are in the `Car` class.
*/

#include "../include/Car.h"
#include <iostream>

using std::cout, std::endl;

// Default Constructor
Car::Car() {
    fuel = 0.0f;
    speed = 0.0f;
    passengers = 0;
    cout << "Default Constructor called." << endl;
}

// Parametized constructor
Car::Car(float amount) {
    fuel = amount;
    speed = 0.0f;
    passengers = 0;
    cout << "Parameterized Constructor called. Fuel set to " << fuel << endl;
}

// Destructor
Car::~Car() {
    cout << "Destructor called. Cleaning up..." << endl;
}

void Car::FillFuel(float fuelAmount) {
    fuel = fuelAmount;
}

void Car::Accelerate() {
    speed++;
    fuel -= 0.5f;
}

void Car::Brake() {
    speed = 0;
}

void Car::AddPassengers(int numberOfPassengers) {
    passengers = numberOfPassengers;
}

void Car::Dashboard() {
    cout << "Fuel: " << fuel << endl;
    cout << "Speed: " << speed << endl;
    cout << "Passengers: " << passengers << endl;
}
