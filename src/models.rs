use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Article {
    pub title: String,
    pub link: String,
    pub time: String,
}
