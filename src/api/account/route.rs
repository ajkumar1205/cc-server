// use crate::api::account::controller::user_signup;
// use actix_web::{web, Handler, HttpResponse};

// pub fn account_config(cfg: &mut web::ServiceConfig) {
//     cfg.service(
//         web::resource("/")
//             .route(web::post().to(|| user_signup))
//             .route(web::head().to(HttpResponse::MethodNotAllowed)),
//     );
// }
