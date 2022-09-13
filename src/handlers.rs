use crate::models::Article;
use crate::response::ResponseBody;
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
    HttpResponse::Ok().json(ResponseBody::new(200, "success", list))
}
