#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use std::sync::Mutex;
use std::collections::HashMap;

use rocket_contrib::json::{JsonValue};
use diesel::pg::PgConnection;
use diesel::Connection;
use dotenv::dotenv;
use std::env;
use models::*;

pub mod controller;
pub mod models;
pub mod schema;

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn connect() -> PgConnection {
    dotenv().ok();

    let connection_string = env::var("CONNECTION_STRING").expect("CONNECTION_STRING must be set");

    PgConnection::establish(&connection_string).expect(&format!("Error connecting to {}", connection_string))
}



fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![
            controller::data_status::new,
            // controller::data_status::update,
            controller::data_status::get,
            controller::data_status::list
        ])
        .register(catchers![not_found])
        .manage(Mutex::new(HashMap::<ID, String>::new()))
}

fn main() {
    rocket().launch();
}