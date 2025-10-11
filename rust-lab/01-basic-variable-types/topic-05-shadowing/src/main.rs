// In Rust, shadowing is when you use the `let` keyword to declare a new
// variable that uses the same name as a variable that already exists.
//
// 1. Shadowing creates a completely new variable so you can change the type.
// 2. The variable that is shadowed will replace the old one until the
// shadowed variable goes out of scope.
// A scope is everything inside a pair of curly braces `{}`
//
// Shadowing is quality of life feature in Rust. 
// It is used when you want to perform an action at a point in time 
// but you don't want to come up with a new variable name.
//
// A practical use of shadowing is when have a function that needs 
// to process some input until that input is in a usable form.

fn main() {
    //_________________________________________________________________________
    // SECTION: Example 1 - Shadowing within a scope

    let a: u8 = 10;
    let b: u8 = 60;

    let total: u8 = a + b;
    println!("{a} + {b} = {total}");
    // 10 + 60 = 70

    {
        let c: f64 = 20.25;
        let d: f64 = 10.18;
        println!("c is {c}");
        println!("d is {d}");

        // Shadowing is taking place here.
        // I am creating a new variable called `total` that is an f64
        let total: f64 = c + d;
        println!("{c} + {d} = {total}")
        // 20.25 + 10.18 = 30.43

    } // This is where the scope of `c` and `d` ends.
    
    // If you try to interact with `c` and `d` after this, 
    // you will get a compile time error.

    // After shadowing is done, the variable name `total` is given back to
    // the original variable called `total`
    println!("total is {total}");
    // total is 70

    //_________________________________________________________________________
}
