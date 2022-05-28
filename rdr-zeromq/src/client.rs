use crate::traits::Client;
use zeromq::prelude::*;
use zeromq::{ZmqMessage, ZmqResult};
use rdr_core::prelude::{encoded_img, detected_armor};
use async_trait::async_trait;
use crate::def_client;

/*struct EncodedImgClient {
    socket: zeromq::SubSocket,
}

#[async_trait]
impl Client for EncodedImgClient {
    type TMessage = EncodedImg;

    async fn new(endpoint: &str) -> Self {
        EncodedImgClient {
            socket: Self::create_socket(endpoint).await,
        }
    }

    async fn socket_recv(&mut self) -> ZmqResult<ZmqMessage> {
        self.socket.recv().await
    }

    async fn socket_subscribe(&mut self, topic: &str) -> ZmqResult<()> {
        self.socket.subscribe(topic).await
    }
}*/

def_client!(EncodedImgClient, encoded_img::EncodedImg);
def_client!(DetectedArmorClient, detected_armor::DetectedArmor);
