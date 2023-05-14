use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::{
    configuration::get_configuration,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() {
    let config = get_configuration().expect("Failed to parse config.");

    init_subscriber(get_subscriber("zero2prod", "info"));

    let addr = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(addr).unwrap();

    let db_pool = PgPool::connect(&config.database.database_connection_string())
        .await
        .expect("Failed to connect to database.");

    run(listener, db_pool).await.unwrap();
}
