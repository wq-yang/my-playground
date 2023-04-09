use axum::{
    extract::{MatchedPath, State},
    http::Request,
    routing::{get, post, IntoMakeService},
    Form, Router, Server,
};
use chrono::Utc;
use hyper::{server::conn::AddrIncoming, StatusCode};
use serde::Deserialize;
use sqlx::PgPool;
use std::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::info_span;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct SubscriptionForm {
    pub name: String,
    pub email: String,
}
async fn health_check() -> StatusCode {
    StatusCode::OK
}
async fn subscriptions(
    State(pool): State<PgPool>,
    Form(form): Form<SubscriptionForm>,
) -> StatusCode {
    match sqlx::query!(
        r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)
            "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now(),
    )
    .execute(&pool)
    .await
    {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub fn run(
    listener: TcpListener,
    pool: PgPool,
) -> Result<Server<AddrIncoming, IntoMakeService<Router>>, std::io::Error> {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscriptions))
        .layer(
            ServiceBuilder::new().layer(TraceLayer::new_for_http().make_span_with(
                |request: &Request<_>| {
                    // Log the matched route's path (with placeholders not filled in).
                    // Use request.uri() or OriginalUri if you want the real path.
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);

                    info_span!(
                        "http_request",
                        method = ?request.method(),
                        matched_path,
                        some_other_field = tracing::field::Empty,
                    )
                },
            )),
        )
        .with_state(pool);

    let server = axum::Server::from_tcp(listener)
        .expect("failed to start server")
        .serve(app.into_make_service());
    Ok(server)
}
