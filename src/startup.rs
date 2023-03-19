use std::{net::TcpListener, sync::Arc};

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
        .with_state(state);

    axum::Server::from_tcp(listener)
        .unwrap()
        .serve(app.into_make_service())
}
