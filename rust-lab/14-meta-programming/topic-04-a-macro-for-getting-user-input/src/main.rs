// macro_rules! input {
//     ($t: ty) => {
//        let 
//     };
// }


fn get_user_input(/*variable type*/) {
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("failed to read input");

    let n: /*variable type*/ = n.trim().parse().expect("invalid input");
}


fn main() {}
