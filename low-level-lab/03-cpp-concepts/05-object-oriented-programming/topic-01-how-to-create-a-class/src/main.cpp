#include "../include/Car.h"
#include <cstdio>

int main() {
    Car redCar;
    redCar.Dashboard();
    printf("\n");
    // Default Constructor called.
    // Fuel: 0
    // Speed: 0
    // Passengers: 0

    Car greenCar(7.2);
    redCar.Dashboard();
    printf("\n");
    // Parameterized Constructor called. Fuel set to 7.2
    // Fuel: 7.2
    // Speed: 0
    // Passengers: 0

    Car blueCar(4);
    blueCar.Accelerate();
    blueCar.Accelerate();
    blueCar.Accelerate();
    blueCar.Accelerate();
    blueCar.Dashboard();
    printf("\n");
    // Parameterized Constructor called. Fuel set to 4
    // Fuel: 2
    // Speed: 4
    // Passengers: 0

    blueCar.Brake();
    blueCar.AddPassengers(1);
    blueCar.Dashboard();
    printf("\n");

    // Destructor called. Cleaning up...
    // Destructor called. Cleaning up...
    // Destructor called. Cleaning up...
}
