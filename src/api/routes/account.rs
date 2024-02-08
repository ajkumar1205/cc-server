use crate::api::account::controller::{user_login, user_signup};
use actix_web::web::{self, ServiceConfig};

pub fn login(cfg: &mut ServiceConfig) {
    cfg.service(web::resource("/login").route(web::post().to(user_login)));
}

pub fn signup(cfg: &mut ServiceConfig) {
    cfg.service(web::resource("/signup").route(web::post().to(user_signup)));
}
