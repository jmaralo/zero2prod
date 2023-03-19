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
async fn valid_subscribers_test() {
    let (addr, _app) = spawn_app();

    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(format!("http://{}/subscriptions", addr))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn invalid_subscribers_test() {
    let (addr, _app) = spawn_app();

    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (body, test_case) in test_cases {
        let response = client
            .post(format!("http://{}/subscriptions", addr))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return 400 on invalid payload ({})",
            test_case
        );
    }
}