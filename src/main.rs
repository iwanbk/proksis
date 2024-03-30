//#[cfg(feature = "pool")]
use rustis::client::PooledClientManager;
use tokio::net::TcpListener;
use tracing_subscriber;

use proksis::server;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let subscriber = tracing_subscriber::fmt()
        // Use a more compact, abbreviated log format
        .compact()
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Display the thread ID an event was recorded on
        .with_thread_ids(true)
        // Don't display the event's target (module path)
        .with_target(false)
        // Build the subscriber
        .finish();
    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber)?;

    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    let mgr = PooledClientManager::new("redis+cluster://127.0.0.1:7001")?;
    let pool = rustis::bb8::Pool::builder()
        .max_size(2000)
        .build(mgr).await?;
    server::run(listener, pool).await;
    Ok(())
}
