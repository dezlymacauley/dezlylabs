/*

ABOUT: The `.iter()` method

The `.iter()` method is a built-in method in Rust that is available to
any data structure that can be iterated over.

The technical term is "any data structure that implements the Iterator trait".

What does "can be iterated over" mean?

let list_of_numbers: Vec<i32> = vec![20, 30, 50];

list_of_numbers can be iterated over.
It contains multiple values and it is possible to access
each individual value.

`.iter()` on its own is used to create an iterator.

Think of this as preparing `list_of_numbers` for any method that can be
chained onto it, to perform an action on each value in list_of_numbers.


*/

// This brings the `Iter` variable type into scope
use std::slice::Iter;

fn main() {
    let list_of_numbers: Vec<i32> = vec![20, 30, 50];

    // NOTE: This iterator creates immutable references to the the values
    // of list_of_numbers. In simple terms it can only be `read-only` chained
    // with methods that perform `read-only` actions on list_of_numbers.

    // If you wanted to chain on a method that modifies each values in
    // list_of_numbers then you need to add the `mut` keyword.
    // let mut iterator_list_of_numbers = list_of_numbers.iter();

    let iterator_list_of_numbers: Iter<'_, i32> = list_of_numbers.iter();
    println!("iterator_list_of_numbers: {:?}", iterator_list_of_numbers);
    // iterator_list_of_numbers: Iter([20, 30, 50])
}
