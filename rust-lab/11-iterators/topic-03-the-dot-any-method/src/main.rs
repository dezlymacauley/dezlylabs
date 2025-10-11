use std::slice::Iter;
fn main() {
    let list_of_numbers: Vec<i32> = vec![21, 2, 29, 20];

    let mut iterator_list_of_numbers: Iter<'_, i32> = list_of_numbers.iter();

    let has_number_below_ten: bool =
        // .any() is a method (an associated function) that accept a closure
        // and returns true or false.
        // A reference to the value of each number in list_of_numbers is
        // passed to the closure so that it can use an i32 value to compare
        // it to 10 which is also an i32 value.

        // If any of the values in list_of_numbers is true for the condition
        // `value < 10`, then has_number_below_ten will be set to true,
        // else it will be set to false.
        iterator_list_of_numbers.any(|&value| value < 10);

    println!(
        "list_of_numbers contain a value that is less than 10? {}",
        has_number_below_ten
    );
    
    // list_of_numbers contain a value that is less than 10? true
}
