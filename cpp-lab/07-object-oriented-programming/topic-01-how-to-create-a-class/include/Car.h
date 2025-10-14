// This ensures that header file is included only once when 
// this project is compiled.
#pragma once

class Car {
private:
    float fuel;
    float speed;
    int passengers;

public:
    // Default Constructor
    // The compiler will automatically call this function when an instance
    // of the class is created.
    Car();

    // Parameterized Constructor
    Car(float amount);

    // Destructor function
    ~Car();

    void FillFuel(float fuelAmount);
    void Accelerate();
    void Brake();
    void AddPassengers(int numberOfPassengers);
    void Dashboard();
};
