use async_trait::async_trait;
use zeromq::{PubSocket, Socket, SubSocket, ZmqMessage, ZmqResult};
use rdr_core::prelude::Message;
use crate::bytes_iterator_reader::BytesIteratorReader;

#[async_trait]
pub trait Server {
    type TMessage: Message;

    async fn create_socket(endpoint: &str) -> PubSocket {
        let mut pub_socket = PubSocket::new();
        pub_socket.bind(endpoint).await.unwrap();
        pub_socket
    }
    async fn new(endpoint: &str) -> Self;
    async fn send(&mut self, msg: &Self::TMessage) -> ZmqResult<()> {
        self.socket_send(ZmqMessage::from(msg.write_to_bytes().unwrap())).await
    }
    async fn socket_send(&mut self, msg: ZmqMessage) -> ZmqResult<()>;
    fn get_socket(&self) -> &PubSocket;
}

#[async_trait]
pub trait Client {
    type TMessage: Message;

    async fn create_socket(endpoint: &str) -> SubSocket {
        let mut sub_socket = SubSocket::new();
        sub_socket.connect(endpoint).await.unwrap();
        sub_socket
    }
    async fn new(endpoint: &str) -> Self;
    async fn socket_recv(&mut self) -> ZmqResult<ZmqMessage>;
    async fn socket_subscribe(&mut self, topic: &str) -> ZmqResult<()>;
    async fn recv(&mut self) -> ZmqResult<Self::TMessage> {
        let msg = self.socket_recv().await?;
        if msg.len() == 1 {
            Ok(Self::TMessage::parse_from_tokio_bytes(msg.get(0).unwrap()).unwrap())
        } else {
            // Ok(T::parse_from_reader(&mut BytesChainIterator { chain: msg.iter() }).unwrap())
            Ok(Self::TMessage::parse_from_reader(&mut BytesIteratorReader::new(msg.iter())).unwrap())
        }
    }
}

// struct BytesChainIterator<'a, T: Iterator<Item=&'a Bytes>> {
//     chain: T,
// }
//
// impl<'a, T: Iterator<Item=&'a Bytes>> BytesChainIterator<'a, T> {
//     fn new(chain: T) -> BytesChainIterator<'a, T> {
//         BytesChainIterator {
//             chain
//         }
//     }
//     // fn from_msg(msg: &ZmqMessage) -> BytesChainIterator<std::collections::vec_deque::Iter<Bytes>> {
//     //     BytesChainIterator::new(msg.iter())
//     // }
// }
//
// impl<'a, T: Iterator<Item=&'a Bytes>> Read for BytesChainIterator<'a, T> {
//     fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
//         self.chain.next() // Two kind of implementation:
//
//             // .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::UnexpectedEof))
//             // .and_then(|bytes| bytes.slice(..).reader().read(buf)) // Throw Error when out of Bytes
//
//             .map_or(Ok(0), |bytes| bytes.slice(..)
//                 .reader().read(buf)) // Ok(0) when out of Bytes, consist with Buf::Reader
//     }
// }
