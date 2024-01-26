use migration::{Migrator, MigratorTrait};
use sea_orm::*;

use crate::settings::Settings;

pub async fn set_up_db() -> Result<DatabaseConnection, DbErr> {
    // getting database url from env
    let config = Settings::get_config();
    let db_config = match config {
        Ok(t) => t.database,
        Err(e) => panic!("error in fetching config settings : {:?}", e),
    };
    let database_url = db_config.url;
    let db_name = db_config.database_name;

    // pool of database connection
    let db_connection = Database::connect(&database_url).await?;

    let db_connection = match db_connection.get_database_backend() {
        DbBackend::Sqlite => db_connection,
        DbBackend::MySql => {
            db_connection
                .execute(Statement::from_string(
                    db_connection.get_database_backend(),
                    format!("CREATE DATABASE IF NOT EXISTS `{}`;", db_name),
                ))
                .await?;

            let url = format!("{}/{}", database_url, db_name);
            Database::connect(&url).await?
        }
        DbBackend::Postgres => {
            db_connection
                .execute(Statement::from_string(
                    db_connection.get_database_backend(),
                    format!("DROP DATABASE IF EXISTS \"{}\";", db_name),
                ))
                .await?;
            db_connection
                .execute(Statement::from_string(
                    db_connection.get_database_backend(),
                    format!("CREATE DATABASE \"{}\";", db_name),
                ))
                .await?;

            let url = format!("{}/{}", database_url, db_name);
            Database::connect(&url).await?
        }
    };

    // connection migrator
    Migrator::up(&db_connection, None).await?;

    Ok(db_connection)
}
