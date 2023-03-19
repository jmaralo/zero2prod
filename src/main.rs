use std::net::TcpListener;

use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() {
    let config = get_configuration().expect("Failed to parse config.");

    let addr = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(addr).unwrap();
    run(listener).await.unwrap();
}
