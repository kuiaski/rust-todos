use std::sync::Mutex;
use std::collections::HashMap;

// use crate::schema::*;

// use serde::Serialize;
// use chrono::NaiveDateTime;

// #[derive(Debug, Queryable, Serialize)]
// pub struct Category {
//     pub id: i32,
//     pub name: String,
//     pub created_at: NaiveDateTime,
//     pub deleted_at: Option<NaiveDateTime>
// }

// #[derive(Debug, Insertable, AsChangeset)]
// #[table_name="category"]
// pub struct NewCategory {
//     pub name: String
// }

// #[derive(Debug, Queryable, Serialize, Deserialize)]
// pub struct DataStatus {
//     pub id: i32,
//     pub name: String,
//     pub created_at: NaiveDateTime,
//     pub deleted_at: Option<NaiveDateTime>
// }

// #[derive(Debug, Insertable, AsChangeset)]
// #[table_name="data_status"]
// pub struct NewDataStatus {
//     pub name: String
// }

// #[derive(Debug, Queryable, Serialize)]
// pub struct Label {
//     pub id: i32,
//     pub name: String,
//     pub color_hex: Option<String>,
//     pub created_at: NaiveDateTime,
//     pub deleted_at: Option<NaiveDateTime>
// }

// #[derive(Debug, Insertable, AsChangeset)]
// #[table_name="label"]
// pub struct NewLabel {
//     pub name: String,
//     pub color_hex: Option<String>
// }

// #[derive(Debug, Queryable, Serialize)]
// pub struct List {
//     pub id: i32,
//     pub name: String,
//     pub created_at: NaiveDateTime,
//     pub deleted_at: Option<NaiveDateTime>
// }

// #[derive(Debug, Insertable, AsChangeset)]
// #[table_name="list"]
// pub struct NewList {
//     pub name: String
// }

// #[derive(Debug, Queryable, Serialize)]
// pub struct Todo {
//     pub id: i32,
//     pub title: String,
//     pub description: String,
//     pub start_at: NaiveDateTime,
//     pub end_at: NaiveDateTime,
//     pub id_category: Option<i32>,
//     pub id_label: Option<i32>,
//     pub id_list: Option<i32>,
//     pub id_data_status: Option<i32>,
//     pub created_at: NaiveDateTime,
//     pub deleted_at: Option<NaiveDateTime>
// }

// #[derive(Debug, Insertable, AsChangeset)]
// #[table_name="todo"]
// pub struct NewTodo {
//     pub title: String,
//     pub description: String,
//     pub start_at: NaiveDateTime,
//     pub end_at: NaiveDateTime,
//     pub id_category: Option<i32>,
//     pub id_label: Option<i32>,
//     pub id_list: Option<i32>,
//     pub id_data_status: Option<i32>
// }

// The type to represent the ID of a message.
pub type ID = usize;

// We're going to store all of the messages here. No need for a DB.
pub type MessageMap = Mutex<HashMap<ID, String>>;

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id: Option<ID>,
    pub contents: String
}