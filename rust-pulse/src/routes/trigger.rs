use actix_web::{get, HttpResponse, Responder};

#[get("/trigger")]
pub async fn trigger_go_server() -> impl Responder {
    HttpResponse::Ok().body("Triggered Go Server")
}
