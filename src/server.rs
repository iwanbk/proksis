use tokio::net::TcpListener;

use crate::proksis;

pub async fn run(listener: TcpListener) {
    let p = proksis::Proksis::new(listener);

    p.run().await;
}