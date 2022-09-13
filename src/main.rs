mod config;
mod handlers;
mod models;
mod response;

use actix_web::{web, App, HttpServer};
use handlers::{get_acticles, index};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://localhost:3000/");

    let app = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/articles", web::get().to(get_acticles))
    });


    app.bind(("0.0.0.0", 3000))?
    .run()
    .await
}

