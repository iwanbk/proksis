use tokio::net::TcpListener;

use proksis::server;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    server::run(listener).await;
}
