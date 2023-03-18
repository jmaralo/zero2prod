use std::net::TcpListener;

use tokio::task::JoinHandle;
use zero2prod::run;

#[tokio::test]
async fn health_check_works() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();

    let _app = spawn_app(listener);

    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://127.0.0.1:{}/health_check", port))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app(listener: TcpListener) -> JoinHandle<Result<(), hyper::Error>> {
    tokio::spawn(run(listener))
}
