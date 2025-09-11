fn main() {
    //_________________________________________________________________________
    // SECTION: How to create a string in Rust

    let msg_1: String = String::from("Kakashi: Time to train");
    println!("message_one: {msg_1}");
    // message_one: Kakashi: Time to train
    
    //_________________________________________________________________________
    // SECTION: Escape sequences

    // How to create a string that contains double qoutes

    // You use `\"`.
    // This is called an escape sequence.

    let msg_2: String = String::from("Kakashi: \"Time to train\"");
    println!("message_two: {msg_2}");
    // message_two: Kakashi: "Time to train"

    //_________________________________________________________________________
    // SECTION: Raw string literal

    // The advantage is that you don't need any escape sequences.
    // Every character will be treated as a string.

    // The syntax is `r#"Your message"#`
    let msg_3: String = String::from(r#"Kakashi: "Time to train""#);
    println!("message_three: {msg_3}");
    // message_three: Kakashi: "Time to train"

    // This can be quite convinient
    // for displaying JSON (JavaScript Object Notation)

    let single_line_json_data: String = String::from(
        r#"{"clan": "Uchiha", "name": "Sasuke", "age": 33, "weapon": "sword" }"#,
    );

    println!("single_line_json_data:");
    println!("{single_line_json_data}");
    // single_line_json_data:
    // {"clan": "Uchiha", "name": "Sasuke", "age": 33, "weapon": "sword" }

    // You have to format it like this because white space matters when
    // you use is raw string literal.
    let multi_line_json_data: String = String::from(
        r#"{
  "clan": "Uchiha",
  "name": "Sasuke",
  "age": 33
}"#,
    );

    println!("multi_line_json_data:");
    println!("{multi_line_json_data}");
    // multi_line_json_data:
    // {
    //   "clan": "Uchiha",
    //   "name": "Sasuke",
    //   "age": 33
    // }

    //_________________________________________________________________________
}
