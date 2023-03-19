use std::net::{SocketAddr, TcpListener};

use tokio::task::JoinHandle;
use zero2prod::run;

/// Spawns a new app and returns the address it is binded to, as well as a join handle for the server
fn spawn_app() -> (SocketAddr, JoinHandle<Result<(), hyper::Error>>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to create listener.");
    (
        listener.local_addr().expect("Failed to get server address"),
        tokio::spawn(run(listener)),
    )
}

#[tokio::test]
async fn health_check_test() {
    let (addr, _app) = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://{}/health_check", addr))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
