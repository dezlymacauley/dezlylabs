use rocket::{post, get, put, delete, catch, catchers, routes};
use rocket::serde::json::{Value, json};
use rocket::response::status;

// This is my custom auth module in src/auth.rs
mod auth;
use auth::BasicAuth;

//_____________________________________________________________________________
// SECTION: Auth

//_____________________________________________________________________________
// SECTION: CRUD Endpoints

//_____________________________________________________________________________
// SUB_SECTION: Create

// `format = "json"` is an http header.
// This means that the payload that is sent to this route must be in 
// json format.
#[post("/rustaceans", format = "json")]
fn create_rustacean() -> Value {
    json!(
        { 
            "id": 3,
            "name": "John Doe",
            "email": "johndoe@gmail.com"
        }
    )
}

//_____________________________________________________________________________
// SUB_SECTION: Read

#[get("/rustaceans")]
fn get_rustaceans(_auth: BasicAuth) -> Value {
    json!(
        [
        { 
            "id": 1,
            "name": "John Doe"
        },
        { 
            "id": 2,
            "name": "Jane Doe"
        },
        ]
    )
}

#[get("/rustaceans/<id>")]
fn view_rustacean(id: i32, _auth: BasicAuth) -> Value {
    json!(
        { 
            "id": id,
            "name": "John Doe",
            "email": "johndoe@gmail.com"
        }
    )
}

//_____________________________________________________________________________
// SUB_SECTION: Update

// #[put("/rustaceans", format = "json") ]
// fn update_rustacean(id: i32) -> Value {
//     json!(
//         { 
//             "id": id,
//             "name": "John Doe",
//             "email": "johndoe@gmail.com"
//         }
//     )
// }

#[put("/rustaceans/<id>", format = "json")]
fn update_rustacean(id: i32, _auth: BasicAuth) -> Value {
    json!(
        {
            "id": id,
            "name": "Updated Name",
            "email": "updated@gmail.com"
        }
    )
}

//_____________________________________________________________________________
// SUB_SECTION: Delete

// The _ is used to silence the compiler warning about the unused parameter.
#[delete("/rustaceans/<_id>")]
fn delete_rustacean(_id: i32, _auth: BasicAuth) -> status::NoContent {
    status::NoContent
}

//_____________________________________________________________________________
// SECTION: Custom Error Messages

#[catch(404)]
fn not_found() -> Value {
    json!("Not found")
}

//_____________________________________________________________________________

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![
            // Create
            create_rustacean,
            
            // Read
            get_rustaceans,
            view_rustacean,

            // Update
            update_rustacean,

            // Delete
            delete_rustacean
        ])
        .register("/", catchers![
            not_found
        ])
        .launch()
        .await;
}
