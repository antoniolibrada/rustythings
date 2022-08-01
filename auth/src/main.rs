use actix_web::{middleware, App, HttpServer};

mod db;
mod errors;
mod handlers;
mod models;

use crate::handlers::get;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().wrap(middleware::Logger::default()).service(get))
        .bind(format!("{}:{}", "127.0.0.1", "8080"))?
        .run()
        .await
}
