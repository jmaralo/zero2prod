use std::net::TcpListener;

use secrecy::ExposeSecret;
use sqlx::PgPool;
use zero2prod::{
    configuration::get_configuration,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() {
    let config = get_configuration().expect("Failed to parse config.");

    init_subscriber(get_subscriber("zero2prod", "info", std::io::stdout));

    let addr = format!("{}:{}", config.application.host, config.application.port);
    let listener = TcpListener::bind(addr).unwrap();

    let db_pool =
        PgPool::connect_lazy(config.database.database_connection_string().expose_secret())
            .expect("Failed to connect to database.");

    run(listener, db_pool).await.unwrap();
}
