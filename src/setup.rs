extern crate dotenv;

use dotenv::dotenv;
use std::env;

use sea_orm::*;

pub async fn set_up_db() -> Result<DatabaseConnection, DbErr> {
    dotenv().ok();

    // getting database url from env
    let database_url =
        env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:collabs-code.db".to_string());
    let db_name = env::var("DB_NAME").unwrap_or_else(|_| "collabs-code".to_string());

    let db = Database::connect(database_url.clone()).await?;

    let db = match db.get_database_backend() {
        DbBackend::Sqlite => db,
        DbBackend::MySql => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE IF NOT EXISTS `{}`;", db_name),
            ))
            .await?;

            let url = format!("{}/{}", database_url, db_name);
            Database::connect(&url).await?
        }
        DbBackend::Postgres => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("DROP DATABASE IF EXISTS \"{}\";", db_name),
            ))
            .await?;
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE \"{}\";", db_name),
            ))
            .await?;

            let url = format!("{}/{}", database_url, db_name);
            Database::connect(&url).await?
        }
    };

    Ok(db)
}
