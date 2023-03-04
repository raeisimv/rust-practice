use std::net::TcpListener;
use sqlx::{PgPool};
use tokio;
use zero2prod::conf;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    let conf = conf::get_configuration()
        .expect("failed to get_configuration");

    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("failed to find a random port to bind");
    let port = listener.local_addr().unwrap().port();

    let db_pool = PgPool::connect(&conf.database.conn_string())
        .await
        .expect("failed to connect to pg");

    let server = zero2prod::startup::run(listener, db_pool.clone())
        .expect("failed to bind address");
    let _ = tokio::spawn(server);
    TestApp {
        db_pool,
        address: format!("http://127.0.0.1:{port}"),
    }
}
