use sqlx::PgPool;
use std::net::{SocketAddr, TcpListener};
use subscription::configuration::{get_configuration, get_connect_option};
use subscription::startup::run;

#[tokio::main]
async fn main() {
    let configs = get_configuration().expect("Failed to load config");
    let addr = SocketAddr::from(([127, 0, 0, 1], configs.app_port));
    let listener = TcpListener::bind(addr).expect("failed to bind port 8080");

    let db_pool = PgPool::connect_with(get_connect_option().expect("failed to get connect option"))
        .await
        .expect("failed to connect to pgpool");
    run(listener, db_pool)
        .expect("Failed to bind address")
        .await
        .unwrap()
}
