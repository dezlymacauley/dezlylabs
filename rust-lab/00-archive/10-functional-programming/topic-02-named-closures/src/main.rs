/*

ABOUT: Named Closures

Named closures are a syntactic style that can be used to simplify a regular
function.

Named closures are declared with the `let` keyword like variables.

The right side of a named closure is an anonymous function,
which is a function that has no name.


ABOUT: How to convert a regular function
into an anonymous function (aka a closure)

The syntax of an anonymous function is this:

let variable_name = | parameters | -> return-type-of-function {
    // logic of function
};


I'll start with this regular function

fn double_attack(attack: i32) -> i32 {
    attack * 2
}

This can be converted to a named closure:

let double_attack = |attack: i32| -> i32 {
    attack * 2
};


ABOUT: How to simplify an anonymous function further

If your closure returns a simple expression, you can leave out the return
type of the function and let Rust infer the return type.

This is safe to do, and Rust will inform you when it can't infer
(safely predict) what type of variable the return type will be.

I've removed the `-> i32` part

let double_attack = |attack: i32| {
    attack * 2
};

ABOUT: How to simplify an annoymous function even further...

When using an anonymous function, the curly braces `{}` are only needed
when the logic of the anonymous function contains multiple lines.

So this is the end result.

let double_attack = |attack: i32| attack * 2;

*/

fn main() {
    // Standard named closure syntax
    let double_attack = |attack: i32| -> i32 { attack * 2 };

    // Simplified named closure syntax
    let thriple_attack = |attack: i32| attack * 3;

    // Super simplified named closure syntax,
    // Allow Rust to infer the parameter type from usage
    let thriple_attack_plus_20 = |attack| (attack * 3) + 20;

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

// This is a higher-order order function
fn calc_final_attack(attack: i32, boost_attack_fn: fn(i32) -> i32) -> i32 {
    let total_damage: i32 = boost_attack_fn(attack);
    total_damage
}
