use std::slice::Iter;

fn main() {

    let list_of_numbers: Vec<i32> = vec![20, 30, 50];

    // NOTE: You must create an iterator and then use `.next()`

    // If you try to use `.next()` directly like this:
    // let second_number = list_of_numbers.iter().next();

    // You will get 20 which is wrong. This is because the example above will
    // create a new iterator each time.

    let mut iterator_list_of_numbers: Iter<'_, i32> = list_of_numbers.iter();

    let first_number = iterator_list_of_numbers.next();
    let second_number = iterator_list_of_numbers.next();
    let third_number = iterator_list_of_numbers.next();

    let fourth_number= iterator_list_of_numbers.next();
    
    println!("first_number: {first_number:?}");
    println!("second_number: {second_number:?}");
    println!("third_number: {third_number:?}");
    println!("fourth_number: {fourth_number:?}");

    // first_number: Some(20)
    // second_number: Some(30)
    // third_number: Some(50)
    // fourth_number: None
}
