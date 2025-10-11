/*

ABOUT: References

The symbol `&` means reference in Rust.

A reference is simply a memory address (The location of where a variable is
stored in the memory of the operating system that it is running on).

The symbol `&` is used to create a reference. The reference does not own
the data, it simply borrows it.

*/

fn main() {
    let user_age: i8 = 18;
    println!("value of user_age: {user_age}");
    println!("user_age is stored at: {:p}", &user_age);

    // user_age: 18
    // user_age is stored at: 0x7ffe8e7f6a9f

    let ref_to_user_age: &i8 = &user_age;

    // Remeber to add `:p` to print the memory address
    println!("ref_to_user_age: {ref_to_user_age:p}");
    // ref_to_user_age: 18

    // Remeber to add the `*` 
    // This is the dereference operator, to get the 
    // the value of the reference.
    println!("value of ref_to_user_age: {}", *ref_to_user_age)

    // value of user_age: 18
    // user_age is stored at: 0x7ffc7740d3c7
    // ref_to_user_age: 0x7ffc7740d3c7
    // value of ref_to_user_age: 18

}
