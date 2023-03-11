use std::net::TcpListener;
use secrecy::ExposeSecret;
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

    let db_pool = PgPool::connect_lazy(conf.database.conn_string().expose_secret())
        .expect("failed to connect to postgres");
    tracing::info!("connect to pg_pool: {}", conf.database.database_name);

    let address = format!("{}:{}", conf.application.host, conf.application.port);
    tracing::info!("server is starting at: {}", address);

    let listener = TcpListener::bind(address)
        .expect("cannot find an TcpListener to bind");

    zero2prod::startup::run(listener, db_pool)?.await
}
