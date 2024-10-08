use actix_web::{post, web, HttpResponse, Responder};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};

use crate::state::AppState;

#[derive(Serialize, Deserialize)]
struct GoroutinePayload {
    identifier: String,
    timestamp: String,
}

#[post("/receive")]
pub async fn receive_goroutine_data(
    payload: web::Json<GoroutinePayload>,
    state: web::Data<AppState>
) -> impl Responder {
    let mut conn = state.redis.lock().await.get_multiplexed_async_connection().await.unwrap();
    let _: () = conn.rpush(
        &payload.identifier,
        &payload.timestamp
    ).await.unwrap();

    HttpResponse::Ok().body("Data received and stored in Redis")
}
