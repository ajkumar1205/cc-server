mod api;
mod db_connection;
mod handler;
mod settings;

use crate::api::account::route::{login, signup};
use db_connection::set_up_db;
use settings::Settings;

use actix_web::{middleware::Logger, web, App, HttpServer};
use sea_orm::DbConn;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "warn");
    std::env::set_var("RUST_BACKTRACE", "1");
    // env_logger::init();
    env_logger::builder()
        .format_timestamp(None) // Do not include timestamps
        .init();

    // confg settings like host and port
    let config = Settings::get_config();
    let server_config = match config {
        Ok(t) => t.server,
        Err(e) => panic!("error in fetching config settings : {:?}", e),
    };

    // pool of db connection. It also connections migrator. -> (db_connection.rs)
    let db_connection: DbConn = match set_up_db().await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };

    // instance of appilcation data
    let db_connection = web::Data::new(db_connection);

    println!("\n \n Running server at http://127.0.0.1:8080/ \n");

    HttpServer::new(move || {
        // let logger = Logger::default();
        let logger = Logger::new("%a %t \"%r\" %s %b %D");

        App::new()
            .wrap(logger)
            .app_data(db_connection.clone()) // Register the application state of data-connection pool
            .service(web::scope("/api").configure(login).configure(signup))
    })
    .bind((server_config.host, server_config.port))?
    .run()
    .await
}
