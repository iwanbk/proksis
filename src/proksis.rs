use std::sync::Arc;

use async_trait::async_trait;
use pingora::apps::ServerApp;
use pingora::protocols::Stream;
use pingora::server::ShutdownWatch;
use pingora::services::listening::Service;

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
        /*let mut buf = [0; 1024];
        loop {
            let n = stream.read(&mut buf).await.unwrap();
            if n == 0 {
                println!("session closing");
                return None;
            }
            stream.write_all(&buf[0..n]).await.unwrap();
            stream.flush().await.unwrap();
        }*/
        let mut conn = connection::Connection::new(stream);
        loop {
            let frame = match conn.read_frame().await {
                Ok(ok_frame) => match ok_frame {
                    Some(frame) => frame,
                    None => return None,
                }
                Err(_) => return None,
            };

            let resp = match Command::from_frame(frame).unwrap() {
                Command::Set(_) => {
                    //let val_str = std::str::from_utf8(&cmd.value()).unwrap();
                    //self.set(cmd.key(), val_str).await
                    Frame::Error("ok".to_string())
                }
                Command::Get(_) => {
                    //self.get(cmd.key().to_string()).await
                    Frame::Error("ok".to_string())
                }
                cmd => panic!("unimplemented command {:?}", cmd),
            };
            conn.write_frame(&resp).await.unwrap();
        }
    }
}

pub fn proksis_service() -> Service<Proksis> {
    Service::new("Echo Service".to_string(), Proksis::new())
}