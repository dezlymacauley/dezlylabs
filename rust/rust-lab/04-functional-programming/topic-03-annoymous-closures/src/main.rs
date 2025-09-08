/*

ABOUT: Annoymous Closures

These are functions that don't have a name.
They are created inline for a specific purpose.

The most common use case is to be passed into a higher-order function.

NOTE: This is the original code where named closures are used.

fn main() {
    let double_attack = |attack: i32| -> i32 { attack * 2 };
    let thriple_attack = |attack: i32| attack * 3;
    let thriple_attack_plus_20 = |attack| (attack * 3) + 20;

    let first_attack: i32 = calc_final_attack(10, double_attack);
    println!("first_attack: {first_attack}");

    let second_attack: i32 = calc_final_attack(10, thriple_attack);
    println!("second_attack: {second_attack}");

    let third_attack: i32 = calc_final_attack(10, thriple_attack_plus_20);
    println!("third_attack: {third_attack}");
}

fn calc_final_attack(attack: i32, boost_attack_fn: fn(i32) -> i32) -> i32 {
    let total_damage: i32 = boost_attack_fn(attack);
    total_damage
}


*/

fn main() {
    let first_attack: i32 = calc_final_attack(10, |attack: i32| attack * 2);
    let second_attack: i32 = calc_final_attack(10, |attack: i32| attack * 3);
    let third_attack: i32 = calc_final_attack(10, |attack| (attack * 3) + 20);

    println!("first_attack: {first_attack}");
    println!("second_attack: {second_attack}");
    println!("third_attack: {third_attack}");
}

fn calc_final_attack(attack: i32, boost_attack_fn: fn(i32) -> i32) -> i32 {
    let total_damage: i32 = boost_attack_fn(attack);
    total_damage
}
