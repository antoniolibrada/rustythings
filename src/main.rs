mod config;
mod db;
mod model;

use crate::config::AppConfig;

use actix_files::Files;
use actix_web::{get, middleware, post, put, web, App, HttpResponse, HttpServer};
use db::DataConect;
use dotenv::dotenv;
use model::*;

struct AppState {
    pub conn: DataConect,
}

#[get("")]
async fn list(state: web::Data<AppState>) -> HttpResponse {
    let res = state.conn.list();
    match res {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(err) => HttpResponse::InternalServerError().json(ErrorDTO {
            message: err.to_string(),
        }),
    }
}
#[get("/{id}")]
async fn get(params: web::Path<u32>, state: web::Data<AppState>) -> HttpResponse {
    let id = params.into_inner();
    let res = state.conn.get(&id);
    match res {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(err) => HttpResponse::InternalServerError().json(ErrorDTO {
            message: err.to_string(),
        }),
    }
}

#[post("")]
async fn add(state: web::Data<AppState>, todo: web::Json<DTOAddTodoInput>) -> HttpResponse {
    let res = state.conn.add(&todo.title);
    match res {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(err) => HttpResponse::InternalServerError().json(ErrorDTO {
            message: err.to_string(),
        }),
    }
}

#[put("/{id}")]
async fn update(
    params: web::Path<u32>,
    state: web::Data<AppState>,
    todo: web::Json<DTOUpdateTodoInput>,
) -> HttpResponse {
    let id = params.into_inner();
    let res = state.conn.update(&todo.title, &todo.completed, &id);
    match res {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(err) => HttpResponse::InternalServerError().json(ErrorDTO {
            message: err.to_string(),
        }),
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
            .service(
                web::scope("/todo")
                    .service(list)
                    .service(get)
                    .service(add)
                    .service(update),
            )
            .service(Files::new("/dist", "./dist").prefer_utf8(true))
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
