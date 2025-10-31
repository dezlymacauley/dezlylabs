// This is a  macro that can accept variables

macro_rules! add_numbers {
    // The syntax is:
    // $parameter_name: indent, $another_parameter_name: indent
    // `indent` means any valid Rust indentify.
    // E.g. A variable name, function name etc
    ($num1: ident, $num2: ident) => {
        $num1 + $num2
    };
}

fn main() {

    let num_one: i32 = 5;
    let num_two: i32 = 10;
    
    let total = add_numbers!(num_one, num_two);

    println!("total: {total}");
    // total: 15
}
