use actix_web::{error, HttpResponse};
use failure::Fail;
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

#[derive(Fail, Debug)]
#[allow(dead_code)]
pub enum BusinessError {
    #[fail(display = "字段验证失败: {}", field)]
    ValidationError { field: String },
    #[fail(display = "服务器错误，请稍后重试.")]
    InternalError,
}

impl error::ResponseError for BusinessError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            BusinessError::ValidationError { .. } => {
                let res = Response::err(10001, &self.to_string());
                HttpResponse::BadRequest().json(res)
            }
            _ => {
                let res = Response::err(10000, &self.to_string());
                HttpResponse::InternalServerError().json(res)
            }
        }
    }
}
