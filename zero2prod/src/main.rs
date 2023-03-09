use std::net::TcpListener;
use sqlx::{PgPool};
use tokio;
use zero2prod::{self, conf, telemetry};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = telemetry::get_subscriber(
        "zero2prod".into(),
        "info".into(),
        std::io::stdout,
    );
    telemetry::init_subscriber(subscriber);

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
