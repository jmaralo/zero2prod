use std::net::TcpListener;

use axum::{
    http::StatusCode,
    routing::{get, IntoMakeService},
    Router, Server,
};
use hyper::server::conn::AddrIncoming;

pub fn run(listener: TcpListener) -> Server<AddrIncoming, IntoMakeService<Router>> {
    let app = Router::new().route("/health_check", get(health_check));

    axum::Server::from_tcp(listener).unwrap().serve(app.into_make_service())
}

async fn health_check() -> StatusCode {
    return StatusCode::OK;
}
