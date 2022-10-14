mod config;
mod handlers;
mod models;
mod response;

use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use handlers::*;
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();

  let config = crate::config::Config::from_env().unwrap();

  let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect("postgres://sayhub:sayhub@127.0.0.1/sayhub")
    .await
    .unwrap();

  println!("pool{:?}", pool);

  println!(
    "Starting server at http://{}:{}/",
    config.server.host, config.server.port
  );

  let app = HttpServer::new(move || {
    App::new()
      .app_data(Data::new(pool.clone()))
      .route("/", web::get().to(index))
      .route("/articles", web::get().to(get_acticles))
      .route("/error", web::get().to(get_error))
      .route("/todos", web::get().to(get_todos))
      .route("/todos", web::post().to(create_todo))
      .route("/todos/{list_id}/items", web::get().to(get_items))
  });

  app
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
