use crate::traits::Client;
use zeromq::prelude::*;
use zeromq::{ZmqMessage, ZmqResult};
use rdr_core::prelude::{encoded_img, detected_armor};
use async_trait::async_trait;
use crate::def_client;

def_client!(EncodedImgClient, encoded_img::EncodedImg);
def_client!(DetectedArmorClient, detected_armor::DetectedArmor);
