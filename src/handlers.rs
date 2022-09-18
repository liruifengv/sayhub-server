use crate::models::Article;
use crate::response::{BusinessError, Response};
use actix_web::{HttpResponse, Responder};

pub async fn index() -> impl Responder {
  HttpResponse::Ok().body("Hello, world!")
}

pub async fn get_acticles() -> impl Responder {
  let mut list = Vec::new();
  list.push(Article {
    title: String::from("aaa"),
    link: String::from("aaa"),
    time: String::from("aaa"),
  });
  let res = Response::ok(list);
  HttpResponse::Ok().json(res)
}

pub async fn get_error() -> Result<HttpResponse, BusinessError> {
  // 可以这样
  // HttpResponse::NotFound().json(Response::err(404, "NotFound"))
  // 也可以调用封装好的业务错误
  // return Err(BusinessError::InternalError);
  let field = String::from("testfield");
  return Err(BusinessError::ValidationError { field: field });
}
