use actix_web::web;

pub mod user;
pub mod torrent;
pub mod category;
pub mod settings;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    user::init_routes(cfg);
    torrent::init_routes(cfg);
    category::init_routes(cfg);
    settings::init_routes(cfg);
}
