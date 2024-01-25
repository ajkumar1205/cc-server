mod setup;
use setup::set_up_db;

use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Responder};

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("hello there ")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    // defined in setup.rs
    let db = match set_up_db().await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };
    // Create an instance of the application state
    let db = web::Data::new(db);

    println!("\n Running server at http://127.0.0.1:8080/ \n");

    HttpServer::new(move || {
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            // Register the application state
            .app_data(db.clone())
            .service(web::scope("/api").route("/hey", web::get().to(manual_hello)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
