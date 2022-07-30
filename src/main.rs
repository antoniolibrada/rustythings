mod config;
mod db;
mod model;

use crate::config::AppConfig;

use actix_files::Files;
use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer};
use db::DataConect;
use dotenv::dotenv;
use model::DTOTodoInput;

struct AppState {
    pub conn: DataConect,
}

#[get("/")]
async fn list(state: web::Data<AppState>) -> HttpResponse {
    let result = state.conn.list();
    HttpResponse::Ok().json(result)
}

#[post("/")]
async fn add(state: web::Data<AppState>, todo: web::Json<DTOTodoInput>) -> HttpResponse {
    let success = state.conn.add(&todo.title);
    match success {
        true => HttpResponse::Ok().finish(),
        false => HttpResponse::InternalServerError().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = AppConfig::from_env().unwrap();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!(
        "Starting server at http://{}:{}",
        config.server.host,
        config.server.port
    );

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                conn: DataConect::new(&config.database.connection_string),
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
