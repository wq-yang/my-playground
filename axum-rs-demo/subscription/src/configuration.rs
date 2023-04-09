use serde::Deserialize;
use sqlx::postgres::PgConnectOptions;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub app_port: u16,
    pub db_settings: DBSettings,
}
#[derive(Deserialize, Debug)]
pub struct DBSettings {
    pub username: String,
    pub password: String,
    pub db_name: String,
    pub port: u16,
    pub host: String,
}

pub fn get_connect_option() -> Result<PgConnectOptions, config::ConfigError> {
    get_configuration().map(
        |Settings {
             db_settings:
                 DBSettings {
                     username,
                     password,
                     host,
                     port,
                     db_name,
                 },
             app_port: _,
         }| {
            PgConnectOptions::new()
                .username(&username)
                .password(&password)
                .host(&host)
                .port(port)
                .database(&db_name)
        },
    )
}
pub fn get_connect_option_default_db() -> Result<PgConnectOptions, config::ConfigError> {
    get_configuration().map(
        |Settings {
             db_settings:
                 DBSettings {
                     username,
                     password,
                     host,
                     port,
                     db_name: _,
                 },
             app_port: _,
         }| {
            PgConnectOptions::new()
                .username(&username)
                .password(&password)
                .host(&host)
                .port(port)
        },
    )
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/config/default.toml");
    config::Config::builder()
        .add_source(config::File::with_name(path))
        .build()?
        .try_deserialize::<Settings>()
}
