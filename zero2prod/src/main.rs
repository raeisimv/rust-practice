use std::net::TcpListener;
use tokio;
use sqlx::postgres::PgPoolOptions;
use zero2prod::{self, conf, telemetry};
use zero2prod::email_client::EmailClient;

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

    let sender_email = conf.email_client.sender()
        .expect("invalid sender_email conf");
    let email_client = EmailClient::new(
        conf.email_client.base_url,
        sender_email
    );

    let db_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(15))
        .connect_lazy_with(conf.database.with_db())
        ;

    tracing::info!("connect to pg_pool: {}", conf.database.database_name);

    let address = format!("{}:{}", conf.application.host, conf.application.port);
    tracing::info!("server is starting at: {}", address);

    let listener = TcpListener::bind(address)
        .expect("cannot find an TcpListener to bind");

    zero2prod::startup::run(listener, db_pool, email_client)?.await
}
