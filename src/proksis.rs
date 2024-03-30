use std::sync::Arc;

use rustis::client::Client;
use rustis::commands::StringCommands;
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{self, Duration};
use tracing::error;

use crate::cmd::Command;
use crate::conn::connection;
use crate::conn::frame::Frame;

pub struct Proksis {
    listener: TcpListener,
}

impl Proksis {
    pub fn new(listener: TcpListener) -> Arc<Self> {
        Arc::new(Proksis {
            listener,
        })
    }
}

impl Proksis {
    pub async fn run(&self) -> Result<(), anyhow::Error> {
        loop {
            let socket = self.accept(&self.listener).await?;
            let mut h = Handler {};

            tokio::spawn(async move {
                if let Err(err) = h.handle(socket).await {
                    error!(cause = ?err, "connection error");
                }
            });
        }
    }
    async fn accept(&self, listener: &TcpListener) -> Result<TcpStream, anyhow::Error> {
        let mut backoff = 1;

        // Try to accept a few times
        loop {
            // Perform the accept operation. If a socket is successfully
            // accepted, return it. Otherwise, save the error.
            match listener.accept().await {
                Ok((socket, _)) => return Ok(socket),
                Err(err) => {
                    if backoff > 64 {
                        // Accept has failed too many times. Return the error.
                        return Err(err.into());
                    }
                }
            }

            // Pause execution until the back off period elapses.
            time::sleep(Duration::from_secs(backoff)).await;

            // Double the back off
            backoff *= 2;
        }
    }
    /*async fn process_new(
        self: &Arc<Self>,
        stream: Stream,
        _shutdown: &ShutdownWatch,
    ) -> Option<Stream> {
        let mut conn = connection::Connection::new(stream);
        let client = match Client::connect("redis+cluster://127.0.0.1:7001").await {
            Ok(client) => client,
            Err(err) => {
                error!("error connect: {err}");
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
                _ => Frame::Error("not supported".to_string()),
            };
            conn.write_frame(&resp).await.unwrap();
        }
    }*/
}

#[derive(Debug)]
struct Handler {}

impl Handler {
    //#[instrument(skip(self))]
    async fn handle(&mut self, socket: TcpStream) -> Result<(), anyhow::Error> {
        let mut conn = connection::Connection::new(socket);
        let client = match Client::connect("redis+cluster://127.0.0.1:7001").await {
            Ok(client) => client,
            Err(err) => {
                error!("error connect: {err}");
                return Ok(());
            }
        };
        loop {
            let frame = match conn.read_frame().await {
                Ok(ok_frame) => match ok_frame {
                    Some(frame) => frame,
                    None => return Ok(()),
                }
                Err(_) => return Ok(()),
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
                _ => Frame::Error("not supported".to_string()),
            };
            conn.write_frame(&resp).await.unwrap();
        }
    }
}

