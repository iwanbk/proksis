use std::sync::Arc;

use bytes::Bytes;
use rustis::bb8::Pool;
use rustis::client::PooledClientManager;
use rustis::commands::StringCommands;
use tokio::io::BufReader;
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{self, Duration};
use tracing::{error, info};

use crate::cmd::Command;
use crate::conn::connection;
use crate::conn::frame::Frame;
use crate::miniresp::conn;

pub struct Proksis {
    listener: TcpListener,
    pool: Arc<Pool<PooledClientManager>>,
}

impl Proksis {
    pub fn new(listener: TcpListener, pool: rustis::bb8::Pool<PooledClientManager>) -> Arc<Self> {
        Arc::new(Proksis {
            listener,
            pool: Arc::new(pool),
        })
    }
}

impl Proksis {
    pub async fn run(&self) -> Result<(), anyhow::Error> {
        loop {
            let socket = self.accept(&self.listener).await?;
            let mut h = Handler {
                pool: self.pool.clone(),
            };

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
}


struct Handler {
    pool: Arc<Pool<PooledClientManager>>,
}

impl Handler {
    async fn handle2(&mut self, socket: TcpStream) -> Result<(), anyhow::Error> {
        let mut conn = conn::Conn::new(BufReader::new(socket));
        let cmd = conn.read_command().await.unwrap();
        let name = cmd.name;
        let key = cmd.key;
        info!("command: {name} {key}");
        Ok(())
    }
    //#[instrument(skip(self))]
    async fn handle(&mut self, socket: TcpStream) -> Result<(), anyhow::Error> {
        let client = self.pool.get().await?;
        let mut conn = connection::Connection::new(socket);


        loop {
            // read frame
            let frame = match conn.read_frame().await {
                Ok(ok_frame) => match ok_frame {
                    Some(frame) => frame,
                    None => return Ok(()), // TODO: handle properly
                }
                Err(_) => return Ok(()), // TODO: handle properly
            };

            // parse the frame
            let command = match Command::from_frame(frame) {
                Ok(cmd) => cmd,
                Err(err) => {
                    // using unwrap is OK here, because failed sending could mean broken connection.
                    // better to close the broken connection
                    conn.write_frame(&Frame::Error("invalid command".to_string())).await.unwrap();
                    info!("invalid command {err}");
                    continue;
                }
            };

            // handle command
            let resp = match command {
                Command::Set(cmd) => {
                    let val_str = std::str::from_utf8(&cmd.value()).unwrap();
                    match client.set(cmd.key(), val_str).await {
                        Ok(_) => Frame::Simple("OK".to_string()),
                        Err(err) => Frame::Error(err.to_string()),
                    }
                }
                Command::Get(cmd) => {
                    let val: rustis::Result<Option<String>> = client.get(cmd.key()).await;
                    match val {
                        Ok(val) => match val {
                            Some(val) => Frame::Bulk(Bytes::from(val)),
                            None => Frame::Null
                        }
                        Err(err) => Frame::Error(err.to_string()),
                    }
                }
                _ => Frame::Error("not supported".to_string()),
            };
            // using unwrap is OK here, because failed sending could mean broken connection.
            // better to close the broken connection
            conn.write_frame(&resp).await.unwrap();
        }
    }
}

