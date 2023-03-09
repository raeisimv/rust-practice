use tokio;
use std::net::TcpListener;
use sqlx::{PgConnection, Connection, PgPool, Executor};
use zero2prod::{conf, conf::DatabaseSettings, telemetry};
use once_cell::sync::Lazy;
use secrecy::ExposeSecret;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

static TRACING: Lazy<()> = Lazy::new(|| {
    let sink = if std::env::var("TEST_LOG").is_ok() {
        std::io::stdout
    } else {
        std::io::sink
    };
    let subscriber = telemetry::get_subscriber(
        "test".into(),
        "debug".into(),
        sink,
    );
    telemetry::init_subscriber(subscriber);
});

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let mut conf = conf::get_configuration()
        .expect("failed to get_configuration");

    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("failed to find a random port to bind");
    let port = listener.local_addr().unwrap().port();

    conf.database.database_name = uuid::Uuid::new_v4().to_string();
    let db_pool = establish_database(&conf.database).await;

    let server = zero2prod::startup::run(listener, db_pool.clone())
        .expect("failed to bind address");
    let _ = tokio::spawn(server);
    TestApp {
        db_pool,
        address: format!("http://127.0.0.1:{port}"),
    }
}

pub async fn establish_database(database_conf: &DatabaseSettings) -> PgPool {
    // Connect and create db
    let mut conn = PgConnection::connect(&database_conf.conn_string_without_db().expose_secret())
        .await
        .expect("failed to establish pg conn");
    conn
        .execute(format!(r#"CREATE DATABASE "{}""#, database_conf.database_name).as_str())
        .await
        .expect("failed to create db on established conn");

    // Migrate
    let db_pool = PgPool::connect(&database_conf.conn_string().expose_secret())
        .await
        .expect("failed to create pg pool");
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("failed to run migration on the establish db");

    db_pool
}