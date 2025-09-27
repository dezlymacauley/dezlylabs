fn main() {
    let x: u8 = 12;

    if x > 10 {
        panic!("x can't be greater than 10");
    }

    // If x is greater than 10 the program will crash immediately and display
    // this message.

    // thread 'main' panicked at 07-enums-and-error-handling/topic-03-explicitly-calling-panic/src/main.rs:5:9:
    // x can't be greater than 10
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

    // This line below not below will not be displayed if x crashes.
    println!("x is {x}");
}
