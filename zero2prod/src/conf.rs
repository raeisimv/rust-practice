use config::{Config, ConfigError, File};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub application_port: u16,
    pub database: DatabaseSettings,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn load_conf() -> Result<Settings, ConfigError> {
    let mut conf = Config::default();
    conf.merge(File::with_name("conf"))?;
    conf.try_into()
}