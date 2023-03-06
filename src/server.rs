use crate::data::{Controller, PacketId};
use crate::data::{ControllerTrait, PacketId::RequestControllerData};
use crate::protocol::OpenRGBStream;
use crate::OpenRGB;
use crate::OpenRGBError;
use crate::DEFAULT_PROTOCOL;

use async_trait::async_trait;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::net::ToSocketAddrs;
use tokio::sync::Mutex;

pub struct OpenRGBServer {
    // controller: Arc<Mutex<Controller>>,
    inner: TcpListener,
}

impl OpenRGBServer {
    pub async fn new<T: ToSocketAddrs>(
        // controller: Box<Controller>,
        addr: T,
    ) -> Result<OpenRGBServer, OpenRGBError> {
        // Call the asynchronous connect method using the runtime.
        let inner = tokio::net::TcpListener::bind(addr).await?;

        println!("OpenRGB server now listening");

        Ok(OpenRGBServer {
            // controller: Arc::new(Mutex::new(*controller)),
            inner,
        })
    }

    pub async fn listen(&self) -> Result<(), OpenRGBError> {
        loop {
            let (socket, _) = self.inner.accept().await?;
            // let controller = self.controller.clone();
            // let foo = controller.lock().await;
            tokio::spawn(async move {
                match Self::handle(socket).await {
                    Ok(()) => {}
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            });
        }
    }

    async fn handle(
        // controller: &Controller,
        mut s: TcpStream,
    ) -> Result<(), OpenRGBError> {
        loop {
            s.handle(DEFAULT_PROTOCOL).await?
        }
    }
}
