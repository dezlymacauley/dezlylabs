/* 
    ___________________________________________________________________________
    cargo add tokio -F full
    
    Use this a add all of the features from the external package tokio
    tokio is an async runtime for Rust.

    Think of tokio as Node.js event loop. It allows you to write asynchonus
    non-blocking code.

    ___________________________________________________________________________
        
    cargo add axum
   
    This is a web server framework for Rust. 
    Think of Axum as the Rust ecosystem's equivalent of Express.js

    ___________________________________________________________________________

    Run `cargo build` once

    Then you can just use `cargo rq` to start the server.

    ___________________________________________________________________________
*/

use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    // Build our app
    let app = Router::new().route("/", get(handler));

    // Bind listener first
    let addr = "127.0.0.1:3001";
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    // Print after binding
    println!("🚀 Server listening on http://{addr}");

    // Start serving
    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello World</h1>")
}
