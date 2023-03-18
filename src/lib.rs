use axum::{http::StatusCode, routing::get, Router};

pub async fn run() {
    // build our application with a single route
    let app = Router::new().route("/health_check", get(health_check));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn health_check() -> StatusCode {
    return StatusCode::OK;
}
