use actix_web::web;

pub mod user;
pub mod torrent;
pub mod category;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    user::init_routes(cfg);
    torrent::init_routes(cfg);
    category::init_routes(cfg);
}
