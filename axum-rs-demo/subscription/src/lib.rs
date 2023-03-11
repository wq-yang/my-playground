use axum::{
    routing::{get, IntoMakeService},
    Router, Server,
};
use hyper::server::conn::AddrIncoming;
use std::net::TcpListener;

async fn health_check() {}

pub fn run(
    listener: TcpListener,
) -> Result<Server<AddrIncoming, IntoMakeService<Router>>, std::io::Error> {
    let app = Router::new().route("/health_check", get(health_check));
    let server = axum::Server::from_tcp(listener)
        .expect("failed to start server")
        .serve(app.into_make_service());
    Ok(server)
}
