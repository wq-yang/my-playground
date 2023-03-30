use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use subscription::{configuration::get_connect_option_default_db, startup::run};

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    // build db
    let db_name = uuid::Uuid::new_v4().to_string();
    let connection_options = get_connect_option_default_db().expect("failed to get connect option");
    let mut connection = PgConnection::connect_with(&connection_options)
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, db_name))
        .await
        .expect("Failed to create database.");

    let db_pool = PgPool::connect_with(connection_options.database(&db_name))
        .await
        .expect("failed to connect to a db pool");
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("Failed to migrate");

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let address = listener.local_addr().unwrap().to_string();
    let server = run(listener, db_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    // We return the application address to the caller!
    TestApp { address, db_pool }
}
