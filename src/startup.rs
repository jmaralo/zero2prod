use std::{net::TcpListener, sync::Arc};
use tower_http::trace::TraceLayer;

use axum::{
    routing::{get, post, IntoMakeService},
    Router, Server,
};
use hyper::server::conn::AddrIncoming;
use sqlx::PgPool;

use crate::routes::{health_check, subscriptions};

pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
) -> Server<AddrIncoming, IntoMakeService<Router>> {
    let state = Arc::new(db_pool);

    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscriptions))
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    tracing::info!("Starting server at {}", listener.local_addr().unwrap());
    axum::Server::from_tcp(listener)
        .unwrap()
        .serve(app.into_make_service())
}
