use diesel::prelude::*;
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};

use crate::models::{DataStatus, ID, Message, MessageMap};
use crate::schema::data_status;

// TODO: This example can be improved by using `route` with multiple HTTP verbs.
#[post("/<id>", format = "json", data = "<message>")]
pub fn new(id: ID, message: Json<Message>, map: State<MessageMap>) -> JsonValue {
    let mut hashmap = map.lock().expect("map lock.");
    if hashmap.contains_key(&id) {
        json!({
            "status": "error",
            "reason": "ID exists. Try put."
        })
    } else {
        hashmap.insert(id, message.0.contents);
        json!({ "status": "ok" })
    }
}

#[put("/<id>", format = "json", data = "<message>")]
pub fn update(id: ID, message: Json<Message>, map: State<MessageMap>) -> Option<JsonValue> {
    let mut hashmap = map.lock().unwrap();
    if hashmap.contains_key(&id) {
        hashmap.insert(id, message.0.contents);
        Some(json!({ "status": "ok" }))
    } else {
        None
    }
}

#[get("/status/<id>")]
pub fn get(id: i32) -> Json<Option<DataStatus>> {
    let all_status = data_status::table
        .select(data_status::all_columns)
        .filter(data_status::id.eq(id).and(data_status::deleted_at.is_null()))
        .first::<DataStatus>(&crate::connect())
        .optional()
        .unwrap();
    
    Json(all_status)
}

#[get("/status")]
pub fn list() -> Json<Vec<DataStatus>> {
    let all_status = data_status::table
        .select(data_status::all_columns)
        .filter(data_status::deleted_at.is_null())
        .load::<DataStatus>(&crate::connect())
        .expect("Could not get data_status");
    
    Json(all_status)
}