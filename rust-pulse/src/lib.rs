use std::net::TcpListener;
use actix_web::{dev::Server, App, HttpServer};

pub mod configurations;
pub mod routes;
pub mod services;
pub mod utils;

pub async fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new().configure(routes::init)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
