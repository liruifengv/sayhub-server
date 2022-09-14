use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Response<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

impl<T> Response<T> {
    pub fn ok(data: T) -> Response<T> {
        Response {
            code: 200,
            message: "success".to_owned(),
            data: Some(data),
        }
    }
}

impl Response<()> {
    pub fn err(error: i32, message: &str) -> Self {
        Response {
            code: error,
            message: message.to_owned(),
            data: None,
        }
    }
}
