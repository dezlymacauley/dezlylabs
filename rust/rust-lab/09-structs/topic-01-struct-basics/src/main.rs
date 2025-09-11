/*

ABOUT: Structs

A struct is a custom data type that groups related variables together
under a single name.

*/

use std::f32;

struct Person {
    citizenship: String,
    name: String,
    age: i32,
    gender: char,
    salary: i32,
}

// An `impl` block is used to create methods (aka associated functions).
// These are functions that can only be used by an instance of a struct.

impl Person {
  
    // It is more idomatic in Rust to use `&self` to refer to an
    // instance of a struct.
    fn compute_taxes(&self) -> f32 {
        (self.salary as f32 / 3.0) * 0.5
    }

}

fn main() {
    let person_one: Person = Person {
        name: String::from("Dezly"),
        citizenship: String::from("Mars"),
        age: 30,
        gender: 'M',
        salary: 150_000,
    };

    println!("Name: {}", person_one.name);
    println!("Citizenship: {}", person_one.citizenship);
    println!("Age: {}", person_one.age);
    println!("Gender: {}", person_one.gender);
    println!("Salary: {}", person_one.salary);

    // This is how you use a method.
    // The `.` syntax is the most commonly used syntax.
    println!("Tax: {}", person_one.compute_taxes());

    // This is the longer form
    // println!("Tax: {}", Person::compute_taxes(&person_one));
}
