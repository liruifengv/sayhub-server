use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseBody<T> {
    pub code: i32,
    pub message: String,
    pub data: T,
}

impl<T> ResponseBody<T> {
    pub fn new(code: i32, message: &str, data: T) -> ResponseBody<T> {
        ResponseBody {
            code,
            message: message.to_string(),
            data,
        }
    }
}
