use crate::traits::Server;
use zeromq::prelude::*;
use zeromq::{ZmqMessage, ZmqResult};
use rdr_core::prelude::*;
use async_trait::async_trait;

pub struct EncodedImgServer {
    socket: zeromq::PubSocket,
}

#[async_trait]
impl Server for EncodedImgServer {
    type TMessage = encoded_img::EncodedImg;

    async fn new(endpoint: &str) -> Self {
        EncodedImgServer {
            socket: Self::create_socket(endpoint).await,
        }
    }

    async fn socket_send(&mut self, msg: ZmqMessage) -> ZmqResult<()> {
        self.socket.send(msg).await
    }
}
