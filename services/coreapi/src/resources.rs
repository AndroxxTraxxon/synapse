use actix_web::web;

pub mod action;

pub fn register_services(cfg: &mut web::ServiceConfig) {
    cfg.service(action::build_scope());
}