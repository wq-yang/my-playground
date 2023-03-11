use std::net::{SocketAddr, TcpListener};
use subscription::run;

#[tokio::main]
async fn main() {
    // let app = Router::new().route("/health_check", get(health_check));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(addr).expect("failed to bind port 8080");
    run(listener)
        // axum::Server::bind(&addr)
        //     .serve(app.into_make_service())
        .expect("failed to run")
        .await
        .unwrap()
}
