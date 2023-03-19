use std::sync::Arc;

use axum::{
    extract::{rejection::FormRejection, State},
    http::StatusCode,
    Form,
};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn subscriptions(
    State(conn): State<Arc<PgPool>>,
    form: Result<Form<FormData>, FormRejection>,
) -> StatusCode {
    if let Ok(Form(data)) = form {
        return match sqlx::query!(
            r#"
            INSERT INTO subscriptions (id, email, name, subscribed_at)
            VALUES ($1, $2, $3, $4)
            "#,
            Uuid::new_v4(),
            data.email,
            data.name,
            Utc::now(),
        )
        .execute(conn.as_ref())
        .await
        {
            Ok(_) => StatusCode::OK,
            Err(e) => {
                println!("Failed to execute query: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };
    }
    StatusCode::BAD_REQUEST
}

#[derive(Deserialize, Debug, Clone)]
pub struct FormData {
    name: String,
    email: String,
}
