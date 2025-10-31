/*

ABOUT: Meta Programming

Meta programming is a language feature that allows you to create code that
can generate other code.

Rust achieves this through the use of macros.

Macros can either be declarative or procedural.

_______________________________________________________________________________
Declarative Macros

macro_rules! macro_name {
    // (pattern to match) => { rules}
    (..) => {};

}

_______________________________________________________________________________

*/

macro_rules! set_health {
    // set_health!() = 100
    () => {
        100
    };
    // set_health!(half), means replace this code with 50 
    // Note that half is not a variable but a pattern.
    // It has no variable type.
    (half) => {
        50
    };
}

fn main() {
    let player_one_health: i32 = set_health!();
    println!("player_one_health: {player_one_health}");

    let player_two_health: i32 = set_health!(half);
    println!("player_two_health: {player_two_health}")

    // player_one_health: 100
    // player_two_health: 50

}
