use actix_web::web;
mod health_check;
mod trigger;
mod receive;

fn register_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check::health_check);
    cfg.service(trigger::trigger_go_server);
}

pub fn init(cfg: &mut web::ServiceConfig) {
    register_routes(cfg);
}
