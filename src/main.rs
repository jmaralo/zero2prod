use std::net::TcpListener;

use zero2prod::run;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    run(listener).await.unwrap();
}
