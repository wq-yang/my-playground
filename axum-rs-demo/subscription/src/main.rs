use sqlx::PgPool;
use std::net::{SocketAddr, TcpListener};
use subscription::configuration::{get_configuration, get_connect_option};
use subscription::startup::run;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_tracing_aka_logging=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

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
