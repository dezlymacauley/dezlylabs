/*

cargo add dotenvy

touch .env

Add this to the file:
PORT_NUMBER=7287

*/

fn main() {
    let _ = dotenvy::dotenv();
    let port_number = std::env::var("PORT_NUMBER")
        .unwrap_or_else(|_| "8080".to_string());

    println!("{port_number}");
}
