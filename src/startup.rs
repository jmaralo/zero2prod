use std::net::TcpListener;

use axum::{
    routing::{get, post, IntoMakeService},
    Router, Server,
};
use hyper::server::conn::AddrIncoming;

use crate::routes::{health_check, subscriptions};

pub fn run(listener: TcpListener) -> Server<AddrIncoming, IntoMakeService<Router>> {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscriptions));

    axum::Server::from_tcp(listener)
        .unwrap()
        .serve(app.into_make_service())
}
