use crate::models::{Article, TodoList};
use crate::response::{BusinessError, Response};
use actix_web::{web, HttpResponse, Responder};
use sqlx::{query, query_as, PgPool};

pub async fn index() -> impl Responder {
  HttpResponse::Ok().body("Hello, world!")
}

pub async fn get_acticles() -> impl Responder {
  let list = vec![Article {
    title: String::from("aaa"),
    link: String::from("aaa"),
    time: String::from("aaa"),
  }];
  let res = Response::ok(list);
  HttpResponse::Ok().json(res)
}

pub async fn get_error() -> Result<HttpResponse, BusinessError> {
  // 可以这样
  // HttpResponse::NotFound().json(Response::err(404, "NotFound"))
  // 也可以调用封装好的业务错误
  // Err(BusinessError::InternalError)
  let field = String::from("testfield");
  Err(BusinessError::ValidationError { field })
}

pub async fn get_todos(db_pool: web::Data<PgPool>) -> Result<HttpResponse, BusinessError> {
  let data = query_as!(TodoList, "select * from todo_list")
    .fetch_all(&**db_pool)
    .await
    .unwrap();
  let res = Response::ok(data);
  Ok(HttpResponse::Ok().json(res))
}
