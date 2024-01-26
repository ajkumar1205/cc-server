use config::{Config, ConfigError, Environment, File};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub server: Server,
    pub database: Database,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Server {
    pub host: String,
    pub port: u16,
    pub ssl_mode: SslMode,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    pub url: String,
    pub database_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all = "snake_case")]
pub enum SslMode {
    Disable,
    Allow,
    Prefer,
    Require,
    VerifyCa,
    VerifyFull,
}

impl Settings {
    pub fn get_config() -> Result<Self, ConfigError> {
        dotenv().ok();

        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name("src/config/default"))
            // Add in the current environment file
            // Default to 'development' env
            // Note that this file is _optional_
            .add_source(File::with_name(&format!("src/config/{}", run_mode)).required(false))
            // Add in a local configuration file
            // This file shouldn't be checked in to git
            .add_source(File::with_name("src/config/local").required(false))
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(Environment::with_prefix("APP"))
            // You may also programmatically change settings
            // .set_override("database.url", "sqlite:collabs-code.db")?
            .build()?;

        // -- let's access our configuration
        // println!("debug: {:?}", s.get_bool("debug"));
        // println!("database: {:?}", s.get::<String>("database.url"));

        // You can deserialize (and thus freeze) the entire configuration as
        // Print out our settings (as a HashMap)

        s.try_deserialize()
    }
}
