use tokio::task::JoinHandle;
use zero2prod::run;

#[tokio::test]
async fn health_check_works() {
    let _app = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:3000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> JoinHandle<Result<(), hyper::Error>> {
    tokio::spawn(run())
}
