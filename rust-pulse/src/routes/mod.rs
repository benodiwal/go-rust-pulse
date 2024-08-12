use actix_web::web;
mod health_check;

fn register_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check::health_check);
}

pub fn init(cfg: &mut web::ServiceConfig) {
    register_routes(cfg);
}
