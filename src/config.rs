// use config::{Config, ConfigError, Environment, File};
// use serde::Deserialize;

// #[derive(Deserialize, Clone, Debug)]
// pub struct ServerConfig {
//     pub Host: String,
//     pub Port: i32,
//     pub database_url: String,
// }

// #[derive(Deserialize)]
// pub struct DevConfig {
//     pub server: ServerConfig,
// }

// // impl DevConfig {
// //     pub fn from_env() -> Result<Self, ConfigError> {
// //         let mut cfg = Config::default();
// //         // cfg.merge(config::Environment::new())?;
// //         // cfg.try_into()
// //     }
// // }
