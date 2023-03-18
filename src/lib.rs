use axum::{
    http::StatusCode,
    routing::{get, IntoMakeService},
    Router, Server,
};
use hyper::server::conn::AddrIncoming;

pub fn run() -> Server<AddrIncoming, IntoMakeService<Router>> {
    let app = Router::new().route("/health_check", get(health_check));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap()).serve(app.into_make_service())
}

async fn health_check() -> StatusCode {
    return StatusCode::OK;
}
