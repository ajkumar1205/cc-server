mod db_connection;
mod settings;

use db_connection::set_up_db;
use settings::Settings;

use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Responder};

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("hello there ")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    // confg settings like host and port
    let config = Settings::get_config();
    let server_config = match config {
        Ok(t) => t.server,
        Err(e) => panic!("error in fetching config settings : {:?}", e),
    };

    // pool of db connection. It also connections migrator. -> (db_connection.rs)
    let db_connection = match set_up_db().await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };

    // instance of appilcation data
    let db_connection = web::Data::new(db_connection);

    println!("\n Running server at http://127.0.0.1:8080/ \n");

    HttpServer::new(move || {
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            // Register the application state
            .app_data(db_connection.clone())
            .service(web::scope("/api").route("/hey", web::get().to(manual_hello)))
    })
    .bind((server_config.host, server_config.port))?
    .run()
    .await
}
