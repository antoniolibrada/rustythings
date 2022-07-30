mod config;
mod db;

use crate::config::Config;

use actix_files::Files;
use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer};
use dotenv::dotenv;

use db::{DataConect, Todo};

struct AppState {
    pub conn: DataConect,
}

#[get("/")]
async fn list(state: web::Data<AppState>) -> HttpResponse {
    let result = state.conn.list();
    HttpResponse::Ok().json(result)
}

#[post("/")]
async fn add(state: web::Data<AppState>, todo: web::Json<Todo>) -> HttpResponse {
    state.conn.add(&todo.title);
    HttpResponse::Ok().json(todo)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = Config::from_env().unwrap();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!(
        "Starting server at http://{}:{}",
        config.server.host,
        config.server.port
    );

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                conn: DataConect::new("TODO.db"),
            }))
            .wrap(middleware::Logger::default())
            .service(web::scope("/todo").service(list).service(add))
            .service(
                Files::new("/", "./static")
                    .prefer_utf8(true)
                    .index_file("index.html"),
            )
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
