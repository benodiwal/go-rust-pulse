use actix_web::{get, HttpResponse, Responder};
use reqwest::Client;

#[get("/trigger")]
pub async fn trigger_go_server() -> impl Responder {

    let client = Client::new();
    let url = "http://localhost:8000/trigger";

    let result = client.get(url).send().await;
    match result {
        Ok(response) => {
            if response.status().is_success() {
                HttpResponse::Ok().body("Go Server Triggered Successfully")
            } else {
                HttpResponse::InternalServerError().body("Failed to Trigger Go Server")
            }
        },
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}
