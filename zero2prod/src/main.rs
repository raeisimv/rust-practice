use std::net::TcpListener;
use sqlx::{PgPool};
use tokio;
use zero2prod;
use zero2prod::conf;
use env_logger::Env;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    log::info!("initializing & reading conf");

    let conf = conf::get_configuration()
        .expect("failed to get_configuration");

    let conn_str = conf.database.conn_string();
    let db_pool = PgPool::connect(&conn_str)
        .await
        .expect("failed to connect to postgres");
    log::info!("connect to pg_pool: {}", conf.database.database_name);

    let address = format!("127.0.0.1:{}", conf.application_port);
    log::info!("server is starting at: {}", address);

    let listener = TcpListener::bind(address)
        .expect("cannot find an TcpListener to bind");

    zero2prod::startup::run(listener, db_pool)?.await
}
