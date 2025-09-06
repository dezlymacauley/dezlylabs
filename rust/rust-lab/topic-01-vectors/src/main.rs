fn main() {

    // How to create an empty Vector
    let mut list_a: Vec<i32> = Vec::new();

    // How to add values to a Vector.
    list_a.push(5);
    list_a.push(2);
    list_a.push(7);

    // You can use the `vec!` Macro to create a Vector and add values to it.
    let list_b: Vec<i32> = vec![20, 35, 10];
}
