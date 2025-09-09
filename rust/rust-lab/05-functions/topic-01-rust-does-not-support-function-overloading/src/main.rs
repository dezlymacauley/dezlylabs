/* 

ABOUT: Rust does not support function overloading

fn triple_it(number: i32) -> i32 {
    number * 3
}

fn triple_it(number: f32) -> f32 {
    number * 3.0
}

fn main() {
    println!("{}", triple_it(3));
}

The code above will not compile because the functions have the same name.

It does not matter if the function signatures are different, 
Rust will not try to infer what function you intend to call.

*/


