use std::net::{SocketAddr, TcpListener};

use sqlx::PgPool;
use zero2prod::{configuration::get_configuration, startup::run};

/// Spawns a new app and returns the application details
async fn spawn_app() -> TestApp {
    let config = get_configuration().expect("Failed to parse config.");

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to create listener.");
    let address = listener.local_addr().unwrap();
    let db_pool = PgPool::connect(&config.database.database_connection_string())
        .await
        .expect("Failed to connect to database.");

    let server = run(listener, db_pool.clone());

    tokio::spawn(server);

    return TestApp {
        address: address,
        _db_pool: db_pool,
    };
}

struct TestApp {
    pub address: SocketAddr,
    pub _db_pool: PgPool,
}

#[tokio::test]
async fn health_check_test() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://{}/health_check", app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
