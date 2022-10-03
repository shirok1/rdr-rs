pub mod message;

pub mod prelude {
    pub use crate::message::*;
    pub use protobuf::Message;
    pub use protobuf::well_known_types::timestamp::Timestamp;
    pub use protobuf::MessageField;
}