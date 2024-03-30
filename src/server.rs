use rustis::client::PooledClientManager;
use tokio::net::TcpListener;

use crate::proksis;

pub async fn run(listener: TcpListener, pool: rustis::bb8::Pool<PooledClientManager>) {
    let p = proksis::Proksis::new(listener, pool);

    p.run().await;
}