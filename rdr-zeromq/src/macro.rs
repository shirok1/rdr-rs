#[macro_export]
macro_rules! def_server {
    ($server_t: ident, $data_t: ty)=>{
        pub struct $server_t{
            socket: zeromq::PubSocket,
        }

        #[async_trait]
        impl Server for $server_t {
            type TMessage = $data_t;
            async fn new(endpoint: &str) -> Self {
                $server_t {
                    socket: Self::create_socket(endpoint).await,
                }
            }
            async fn socket_send(&mut self, msg: ZmqMessage) -> ZmqResult<()> {
                self.socket.send(msg).await
            }
            fn get_socket(&self) -> &zeromq::PubSocket { &self.socket }
        }
    };
}

#[macro_export]
macro_rules! def_client {
    ($client_t: ident, $data_t: ty)=>{
        pub struct $client_t {
            socket: zeromq::SubSocket,
        }

        #[async_trait]
        impl Client for $client_t {
            type TMessage = $data_t;
            async fn new(endpoint: &str) -> Self {
                $client_t {
                    socket: Self::create_socket(endpoint).await,
                }
            }
            async fn socket_recv(&mut self) -> ZmqResult<ZmqMessage> {
                self.socket.recv().await
            }
            async fn socket_subscribe(&mut self, topic: &str) -> ZmqResult<()> {
                self.socket.subscribe(topic).await
            }
        }
    };
}

