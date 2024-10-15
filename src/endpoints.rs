use crate::user_id::UserId;
use crate::AppState;
use actix_web::{get, post, web, HttpResponse, Responder};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(health).service(recommendations);
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[post("/recommendations")]
async fn recommendations(state: web::Data<AppState>, data: web::Json<UserId>) -> impl Responder {
    let matrix = state.matrix.lock().unwrap();
    let recommendations = matrix.get_recommendations(data.user_id);
    HttpResponse::Ok().json(recommendations.unwrap())
}
