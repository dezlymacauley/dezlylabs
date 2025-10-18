// The keyword `const` is used to declare a `constant` in Rust.

// 1. Constants can be declared outside of a function.
// 2. Constants are always immutable (The value can't be changed).
// 3. There is no keyword you can apply to a constant to change this.
//    So you can't declare a variable as `const` and `mut` at the same time.
// 4. The right side of a constant should always be a single value 
// or a constant expression.

const MAX_HEALTH: u16 = 65535;

// A constant expression is a single value or expression
// that can be determined before compile time (Before the program runs).
// There are 24 hours in a day, and each of those hours is 60 minutes.
const MINUTES_IN_A_DAY: u16 = 24 * 60;

//_____________________________________________________________________________

fn main() {
    println!("MAX_HEALTH is: {MAX_HEALTH}");
    //MAX_HEALTH is: 65535

    println!("There are {MINUTES_IN_A_DAY} minutes in a day");
    // There are 1440 minutes in a day
}
