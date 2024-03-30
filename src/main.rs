//#[cfg(feature = "pool")]
use rustis::client::PooledClientManager;
use tokio::net::TcpListener;

use proksis::server;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    println!("Hello, world!");
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    let mgr = PooledClientManager::new("redis+cluster://127.0.0.1:7001")?;
    let pool = rustis::bb8::Pool::builder()
        .max_size(2000)
        .build(mgr).await?;
    server::run(listener, pool).await;
    Ok(())
}
