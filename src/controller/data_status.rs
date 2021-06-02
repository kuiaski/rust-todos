use crate::models::{ID, Message, MessageMap};

use rocket::State;
use rocket_contrib::json::{Json, JsonValue};

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

#[get("/status")]
pub fn getAllStatus() -> Option<Json<Vec<Message>>> {
    let mut data: Vec<Message> = Vec::new();
    data.push(Message{
        id: Some(10),
        contents: String::from("Oi Zé")
    });
    data.push(Message{
        id: Some(11),
        contents: String::from("Oi Oi")
    });
    Some(Json(data))
}