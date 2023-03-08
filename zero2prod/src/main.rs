use std::net::TcpListener;
use sqlx::{PgPool};
use tokio;
use zero2prod::{self, conf};
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or(EnvFilter::new("info"));

    let formatting_layer = BunyanFormattingLayer::new("zero2prod".into(), std::io::stdout);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
        ;
    set_global_default(subscriber).expect("failed to set subscriber globally");

    tracing::info!("initializing & reading conf");

    let conf = conf::get_configuration()
        .expect("failed to get_configuration");

    let conn_str = conf.database.conn_string();
    let db_pool = PgPool::connect(&conn_str)
        .await
        .expect("failed to connect to postgres");
    tracing::info!("connect to pg_pool: {}", conf.database.database_name);

    let address = format!("127.0.0.1:{}", conf.application_port);
    tracing::info!("server is starting at: {}", address);

    let listener = TcpListener::bind(address)
        .expect("cannot find an TcpListener to bind");

    zero2prod::startup::run(listener, db_pool)?.await
}
