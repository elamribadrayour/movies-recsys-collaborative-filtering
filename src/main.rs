mod data_processing;
mod endpoints;
mod matrix;
mod user_id;
use std::sync::Mutex;

use actix_web::{web, App, HttpServer};
use simple_logger::SimpleLogger;

struct AppState {
    matrix: Mutex<matrix::Matrix>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    SimpleLogger::new().init().unwrap();
    let state = web::Data::new(AppState {
        matrix: Mutex::new(data_processing::get_dataset().await.unwrap()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .configure(endpoints::init_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
