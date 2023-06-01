use std::sync::Arc;

use axum::{
    extract::{rejection::FormRejection, State},
    http::StatusCode,
    Form,
};
use chrono::Utc;
use serde::Deserialize;
use sqlx::{postgres::PgQueryResult, PgPool};
use tracing::instrument;
use uuid::Uuid;

use crate::domain::{name::SubscriberName, subscriber::NewSubscriber};

#[instrument(skip(conn), level = "info", fields(req_id = Uuid::new_v4().to_string()))]
pub async fn subscriptions(
    State(conn): State<Arc<PgPool>>,
    form: Result<Form<FormData>, FormRejection>,
) -> StatusCode {
    let Ok(Form(data)) = form else {
        tracing::warn!("Invalid form data.");
        return StatusCode::BAD_REQUEST;
    };

    let subscriber_name = match SubscriberName::parse(data.name) {
        Ok(name) => name,
        Err(err) => {
            tracing::warn!("Error parsing name: {}", err);
            return StatusCode::BAD_REQUEST;
        }
    };

    let subscriber = NewSubscriber {
        name: subscriber_name,
        email: data.email,
    };

    tracing::info!("Saving new subscriber: {:?}", subscriber);

    match insert_subscription(conn, &subscriber).await {
        Ok(_) => {
            tracing::info!("Successfully inserted subscription.");
            StatusCode::OK
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

#[instrument(skip(conn), level = "debug")]
async fn insert_subscription(
    conn: Arc<PgPool>,
    subscriber: &NewSubscriber,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO "subscriptions" (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        subscriber.email,
        subscriber.name.inner_ref(),
        Utc::now(),
    )
    .execute(conn.as_ref())
    .await
}

#[derive(Deserialize, Debug, Clone)]
pub struct FormData {
    name: String,
    email: String,
}
