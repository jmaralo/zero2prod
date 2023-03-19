use std::net::TcpListener;

use axum::{
    extract::rejection::FormRejection,
    http::StatusCode,
    routing::{get, post, IntoMakeService},
    Form, Router, Server,
};
use hyper::server::conn::AddrIncoming;
use serde::Deserialize;

pub fn run(listener: TcpListener) -> Server<AddrIncoming, IntoMakeService<Router>> {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscriptions));

    axum::Server::from_tcp(listener)
        .unwrap()
        .serve(app.into_make_service())
}

async fn subscriptions(form: Result<Form<FormData>, FormRejection>) -> StatusCode {
    if let Ok(Form(data)) = form {
        println!("({}) {}", data.name, data.email);
        return StatusCode::OK;
    }
    StatusCode::BAD_REQUEST
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[derive(Deserialize, Debug, Clone)]
struct FormData {
    name: String,
    email: String,
}
