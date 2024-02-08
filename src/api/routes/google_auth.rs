use crate::api::auth::google::google_openid::{callback, login};
use actix_web::web::{self, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(web::resource("/auth/google_auth/login").route(web::get().to(login)));
    cfg.service(web::resource("/auth/google_auth/callback/").route(web::get().to(callback)));
}
