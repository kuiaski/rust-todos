use diesel::prelude::*;
use rocket_contrib::json::{Json};

use crate::models::{DataStatus, NewDataStatus};
use crate::schema::data_status;

#[post("/status", format = "json", data = "<new_status>")]
pub fn new(new_status: Json<NewDataStatus>) -> Json<DataStatus> {
    let inserted_status = diesel::insert_into(data_status::table)
    .values(&*new_status)
    .returning(data_status::all_columns)
    .get_result(&crate::connect())
    .expect("Error saving new status");

    Json(inserted_status)
}

// #[put("/<id>", format = "json", data = "<message>")]
// pub fn update(id: ID, message: Json<Message>, map: State<MessageMap>) -> Option<JsonValue> {
//     let mut hashmap = map.lock().unwrap();
//     if hashmap.contains_key(&id) {
//         hashmap.insert(id, message.0.contents);
//         Some(json!({ "status": "ok" }))
//     } else {
//         None
//     }
// }

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
        .expect("Could not list data_status");
    
    Json(all_status)
}