use config::{Config, ConfigError, File};
use secrecy::{ExposeSecret, Secret};
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::ConnectOptions;
use sqlx::postgres::{PgConnectOptions, PgSslMode};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub required_ssl: bool,
}

impl DatabaseSettings {
    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mod = match self.required_ssl {
            true => PgSslMode::Require,
            false => PgSslMode::Prefer,
        };
        PgConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .username(&self.username)
            .password(&self.password.expose_secret())
            .ssl_mode(ssl_mod)
    }

    pub fn with_db(&self) -> PgConnectOptions {
        let mut opts = self.without_db().database(&self.database_name);
        opts.log_statements(tracing::log::LevelFilter::Trace);
        opts
    }
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
}

pub enum Environment {
    Development,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Development => "development",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "development" => Ok(Self::Development),
            "production" => Ok(Self::Production),
            unknown => Err(format!(" '{}' is not a supported environment. try using 'development' or 'production'", unknown).into())
        }
    }
}

pub fn get_configuration() -> Result<Settings, ConfigError> {
    let base_path = std::env::current_dir().expect("failed to get current_dir");
    let conf_dir = base_path.join("conf");
    let env: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or("development".into())
        .try_into()
        .expect("failed to parse APP_ENVIRONMENT var")
        ;
    tracing::info!("loading conf for '{}'", env.as_str());

    let conf_filename = conf_dir.join(env.as_str());
    Config::builder()
        .add_source(File::with_name("conf/base.yaml").required(true))
        .add_source(File::from(conf_filename).required(true))
        .add_source(config::Environment::with_prefix("app").separator("__"))
        .build()
        .unwrap()
        .try_deserialize()
}