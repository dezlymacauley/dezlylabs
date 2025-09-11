use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7399").unwrap();

    println!("Server Status: Online");
    println!("Listening for a connection at: http://127.0.0.1:7399");

    for _ in listener.incoming() {
        println!("Connection established!");
    }
}
