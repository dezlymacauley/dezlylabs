fn main() {

    // In Rust, variables are immutable by default.
    // Immutable means that you can't change the value of a variable after
    // you have declared that variable.
    
    // If you want a variable to be mutable,
    // you have to add the keyword `mut` to its name.

    let mut user_age: u8 = 20;
    println!("The value of user_age is: {user_age}");
    // The value of user_age is: 20 

    user_age = 40;
    println!("The value of user_age is: {user_age}");
    // The value of user_age is: 40

}
