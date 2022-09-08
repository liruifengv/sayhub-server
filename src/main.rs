mod articles;
mod response;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use articles::Article;
use response::ResponseBody;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/articles")]
async fn get_acticles() -> impl Responder {
    let mut list = Vec::new();
    list.push(Article {
        title: String::from("aaa"),
        link: String::from("aaa"),
        time: String::from("aaa"),
    });
    HttpResponse::Ok().json(ResponseBody::new("success", list))
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(get_acticles)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
