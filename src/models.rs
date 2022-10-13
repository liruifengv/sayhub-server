use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Article {
  pub title: String,
  pub link: String,
  pub time: String,
}

#[derive(Serialize, Deserialize)]
pub struct TodoList {
  pub id: i32,
  pub title: String,
}

#[derive(Serialize, Deserialize)]
pub struct TodoItem {
  pub id: i32,
  pub title: String,
  pub checked: bool,
  pub list_id: i32,
}
