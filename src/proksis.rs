use std::sync::Arc;

use async_trait::async_trait;
use pingora::apps::ServerApp;
use pingora::protocols::Stream;
use pingora::server::ShutdownWatch;
use pingora::services::listening::Service;
use rustis::client::Client;
use rustis::commands::StringCommands;

use crate::cmd::Command;
use crate::conn::connection;
use crate::conn::frame::Frame;

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
        mut stream: Stream,
        _shutdown: &ShutdownWatch,
    ) -> Option<Stream> {
        let mut conn = connection::Connection::new(stream);
        let client = match Client::connect("redis+cluster://127.0.0.1:7001").await {
            Ok(client) => client,
            Err(err) => {
                println!("error connect: {err}");
                return None;
            }
        };
        loop {
            let frame = match conn.read_frame().await {
                Ok(ok_frame) => match ok_frame {
                    Some(frame) => frame,
                    None => return None,
                }
                Err(_) => return None,
            };

            let resp = match Command::from_frame(frame).unwrap() {
                Command::Set(cmd) => {
                    let val_str = std::str::from_utf8(&cmd.value()).unwrap();
                    match client.set(cmd.key(), val_str).await {
                        Ok(_) => Frame::Simple("OK".to_string()),
                        Err(err) => Frame::Error(err.to_string()),
                    }
                }
                Command::Get(cmd) => {
                    //self.get(cmd.key().to_string()).await
                    match client.get(cmd.key()).await {
                        Ok(val) => Frame::Simple(val),
                        Err(err) => Frame::Error(err.to_string()),
                    }
                }
                cmd => Frame::Error("not supported".to_string()),
            };
            conn.write_frame(&resp).await.unwrap();
        }
    }
}

pub fn proksis_service() -> Service<Proksis> {
    Service::new("Echo Service".to_string(), Proksis::new())
}