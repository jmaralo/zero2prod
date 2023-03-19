use axum::{extract::rejection::FormRejection, http::StatusCode, Form};
use serde::Deserialize;

pub async fn subscriptions(form: Result<Form<FormData>, FormRejection>) -> StatusCode {
    if let Ok(Form(data)) = form {
        println!("({}) {}", data.name, data.email);
        return StatusCode::OK;
    }
    StatusCode::BAD_REQUEST
}

#[derive(Deserialize, Debug, Clone)]
pub struct FormData {
    name: String,
    email: String,
}
