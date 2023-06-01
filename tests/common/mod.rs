use std::{
    env,
    net::{SocketAddr, TcpListener},
};

use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use zero2prod::{
    configuration::{get_configuration, DatabaseSettings, Settings},
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

static TRACING: Lazy<()> = Lazy::new(|| {
    let name = "test";
    let filter = "info";
    if env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(name, filter, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(name, filter, std::io::sink);
        init_subscriber(subscriber);
    }
});

/// Spawns a new app and returns the application details
pub async fn spawn_app() -> TestApp {
    let mut config = get_configuration().expect("Failed to parse config.");
    config.database.database_name = Uuid::new_v4().to_string();

    Lazy::force(&TRACING);

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to create listener.");
    let address = listener.local_addr().unwrap();

    let db_pool = mock_database(config.database.clone()).await;

    let server = run(listener, db_pool.clone());

    tokio::spawn(server);

    TestApp {
        address,
        db_pool,
        config,
    }
}

async fn mock_database(settings: DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect_with(&settings.without_db())
        .await
        .expect("Failed to connect to server.");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, settings.database_name).as_str())
        .await
        .expect("Failed to create database.");

    let pool = PgPool::connect_with(settings.with_db())
        .await
        .expect("Failed to connect to database.");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate database.");

    pool
}

async fn clean_database(settings: DatabaseSettings) {
    let mut connection = PgConnection::connect_with(&settings.without_db())
        .await
        .expect("Failed to connect to server.");

    connection
        .execute(format!(r#"DROP DATABASE "{}";"#, settings.database_name).as_str())
        .await
        .expect("Failed to clean up database.");
}

pub struct TestApp {
    pub address: SocketAddr,
    pub db_pool: PgPool,
    pub config: Settings,
}

pub struct Cleanup(pub Settings);

impl Drop for Cleanup {
    fn drop(&mut self) {
        tokio::spawn(clean_database(self.0.database.clone()));
    }
}
