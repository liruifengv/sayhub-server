use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Article {
    pub title: String,
    pub link: String,
    pub time: String,
}
