use axum::{
    routing::{get, post, IntoMakeService},
    Form, Router, Server,
};
use hyper::server::conn::AddrIncoming;
use serde::Deserialize;
use std::net::TcpListener;

#[derive(Deserialize, Debug)]
struct SubscriptionForm {
    name: String,
    email: String,
}
async fn health_check() {}
async fn subscriptions(Form(form): Form<SubscriptionForm>) {
    println!("the params are: {:?}", form);
}

pub fn run(
    listener: TcpListener,
) -> Result<Server<AddrIncoming, IntoMakeService<Router>>, std::io::Error> {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscriptions));

    let server = axum::Server::from_tcp(listener)
        .expect("failed to start server")
        .serve(app.into_make_service());
    Ok(server)
}
