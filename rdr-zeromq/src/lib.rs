extern crate core;

pub mod traits;

pub mod server;
pub mod client;

mod bytes_iterator_reader;

mod r#macro;

pub mod prelude {
    pub use super::traits::*;
    pub use rdr_core::prelude::*;
}
pub use zeromq::ZmqError;
