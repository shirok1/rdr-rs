use crate::traits::Server;
use zeromq::prelude::*;
use zeromq::{ZmqMessage, ZmqResult};
use rdr_core::prelude::*;
use async_trait::async_trait;
use bytes::Bytes;
use crate::def_server;

def_server!(EncodedImgServer, encoded_img::EncodedImg);
def_server!(DetectedArmorServer, detected_armor::DetectedArmor);

impl EncodedImgServer {
    pub async fn send_img(&mut self, img: Bytes) -> ZmqResult<()> {
        let mut msg = encoded_img::EncodedImg::new();
        msg.timestamp = Some(Timestamp::now()).into();
        // msg.data = Bytes::f;
        msg.data = img;

        self.send(&msg).await
    }
}
