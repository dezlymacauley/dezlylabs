fn main() {
    // `single_value` is a Box smart pointer. 
    // The Box itself lives on the Stack.
    // The `f64` value (0.625) is stored on the Heap.
    let single_value: Box<f64> = Box::new(0.625);
}
