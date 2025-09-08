/*

ABOUT: Programming Paradigms

A programming paradigm is a style of writing code.

Functional programming is a Well-known style even
outside of Rust programming.

Many built-in Rust functions and methods (aka associated functions),
make use of functional programming.

So understanding the concept of functional programming is critical
to being able to read intermediate and advanced Rust code.

ABOUT: Functional Programming

One of the key concepts of functional programming is higher-order functions.

A higher-order function is a function
that accepts another function as an argument.

*/

fn double_attack(attack: i32) -> i32 {
    attack * 2
}

fn thriple_attack(attack: i32) -> i32 {
    attack * 3
}

fn thriple_attack_plus_20(attack: i32) -> i32 {
    (attack * 3) + 20
}

// This is a higher-order order function
fn calc_final_attack(attack: i32, boost_attack_fn: fn(i32) -> i32) -> i32 {
    let total_damage: i32 = boost_attack_fn(attack);
    total_damage
}

// This part of the `boost_attack_fn: fn(i32) -> i32` means that the function
// `calc_final_attack` accepts a function pointer.
//
// So when you use `calc_final_attack` you pass in an i32, the name
// of any function that matches the function signature `fn(i32) -> i32`


fn main() {

    // NOTE: The reason why this works is because the three functions,
    // double_attack, third_attack, and thriple_attack_plus_20,
    // all have the same function signature:
    // fn(i32) -> i32

    let first_attack: i32 = calc_final_attack(10, double_attack);
    println!("first_attack: {first_attack}");
    // first_attack: 20
    
    let second_attack: i32 = calc_final_attack(10, thriple_attack);
    println!("second_attack: {second_attack}");
    // second_attack: 30
    
    let third_attack: i32 = calc_final_attack(10, thriple_attack_plus_20);
    println!("third_attack: {third_attack}");
    // third_attack: 50

}
