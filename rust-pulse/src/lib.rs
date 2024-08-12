use std::{net::TcpListener, sync::Arc};
use actix_web::{dev::Server, web, App, HttpServer};
use state::AppState;
use tokio::sync::Mutex;

pub mod configurations;
pub mod routes;
pub mod state;
pub mod utils;
pub mod logger;
mod middlewares;

pub async fn run(listener: TcpListener) -> Result<Server, std::io::Error> {

    let redis_client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let app_state = web::Data::new(AppState {
        redis: Arc::new(Mutex::new(redis_client))
    });

    let server = HttpServer::new(move || {
        App::new()
        .app_data(app_state.clone())
        .wrap(middlewares::logger::logger_middleware())
        .configure(routes::init)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
