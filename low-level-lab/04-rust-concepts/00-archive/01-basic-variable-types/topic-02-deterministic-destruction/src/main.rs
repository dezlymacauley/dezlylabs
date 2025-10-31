/*

ABOUT: Deterministic Destruction

This refers to order that variable are dropped from scope in Rust.
`Dropped from scope` means that the memory address where a variable is stored
is freed, and given back to the operating system so that it can use that
memory location for something else.

In Rust, local variables are dropped in `reverse order of declaration` when
their scope ends.

In simple terms, LIFO (Last In, First Out).

Rust does this to prevent dangling references.

If a later-declared variable borrows or depends on an earlier one, 
the dependent variable is dropped first.

*/

fn main() {
    let num1: i32 = 10;
    let num2: i32 = 20;
    let num3: i32 = 30;

    println!("num1: {num1}");
    println!("num2: {num2}");
    println!("num3: {num3}");
}   // The scope ends here.
// After the line above, the variables num1, num2, and num3
// can't be accessed because they will be dropped from scope.
//
// They will be dropped from scope in this order:
// num3, num2, num1
