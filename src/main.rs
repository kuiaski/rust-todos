#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use std::sync::Mutex;
use std::collections::HashMap;

use rocket_contrib::json::{JsonValue};
use models::*;

pub mod controller;
pub mod models;

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![
            controller::data_status::new,
            controller::data_status::update,
            controller::data_status::getAllStatus
        ])
        .register(catchers![not_found])
        .manage(Mutex::new(HashMap::<ID, String>::new()))
}

fn main() {
    rocket().launch();
}