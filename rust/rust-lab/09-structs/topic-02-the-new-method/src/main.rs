/*

ABOUT: The `new` method is used to make it convient to create an instance
of a struct.

Note that that it does NOT have to be called `new`, this is just the
most idomatic name in Rust
because methods from the standard library follow it.

E.g.

let list_of_numbers: Vec<i32> = Vec::new();

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
    // Self is the idomatic way of writing this
    // You could use the name of the struct `Person` as well.
    fn new() -> Self {
        Self {
            name: String::from("Dezly"),
            citizenship: String::from("Mars"),
            age: 30,
            gender: 'M',
            salary: 150_000,
        }
    }

    fn print_details(&self) {
        println!("Name: {}", self.name);
        println!("Citizenship: {}", self.citizenship);
        println!("Age: {}", self.age);
        println!("Gender: {}", self.gender);
        println!("Salary: {}", self.salary);
        println!("Tax: {}", self.compute_taxes());
    }

    // It is more idomatic in Rust to use `&self` to refer to an
    // instance of a struct.
    fn compute_taxes(&self) -> f32 {
        (self.salary as f32 / 3.0) * 0.5
    }
}

fn main() {
    let person_one: Person = Person::new();
    person_one.print_details();

    let person_two: Person = Person {
        name: String::from("Jack"),
        citizenship: String::from("Neptune"),
        // This means use the current details of person_one to fill in the
        // rest of the fields.

        // This is the struct update syntax
        ..person_one
    };

    println!();
    person_two.print_details();
}
