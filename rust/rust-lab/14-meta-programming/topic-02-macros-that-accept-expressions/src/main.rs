// This is a  macro that can accept expressions

macro_rules! add_numbers {
    // The syntax is:
    // $parameter_name: expr, $another_parameter_name: expr
    // `expr` means any valid Rust expression
    ($num1: expr, $num2: expr) => {
       $num1 + $num2
    };
}

fn main() {

    // NOTE: !() or ![] will work
    
    // !() is the most common. E.g. println!();
    // ![] is used for generating a list of something. E.g. vec![];
    
    let value_a = add_numbers!(5, 2);
    // let value_a = add_numbers!{5, 2};
    // let value_a = add_numbers![5, 2];

    println!("value_a: {value_a}");
    // value_a: 7
}
