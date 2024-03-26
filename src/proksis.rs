use std::sync::Arc;

use async_trait::async_trait;
use pingora::apps::ServerApp;
use pingora::protocols::Stream;
use pingora::server::ShutdownWatch;
use pingora::services::listening::Service;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Clone)]
pub struct Proksis;

impl Proksis {
    pub fn new() -> Arc<Self> {
        Arc::new(Proksis {})
    }
}

#[async_trait]
impl ServerApp for Proksis {
    async fn process_new(
        self: &Arc<Self>,
        mut io: Stream,
        _shutdown: &ShutdownWatch,
    ) -> Option<Stream> {
        let mut buf = [0; 1024];
        loop {
            let n = io.read(&mut buf).await.unwrap();
            if n == 0 {
                println!("session closing");
                return None;
            }
            io.write_all(&buf[0..n]).await.unwrap();
            io.flush().await.unwrap();
        }
    }
}

pub fn proksis_service() -> Service<Proksis> {
    Service::new("Echo Service".to_string(), Proksis::new())
}