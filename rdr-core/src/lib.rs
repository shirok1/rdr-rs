pub mod message;

pub mod prelude {
    pub use crate::message::*;
    pub use protobuf::Message;
    pub use protobuf::well_known_types::timestamp::Timestamp;
    pub use protobuf::MessageField;
    // pub use protobuf::well_known_types_util::timestamp;
}

// pub mod util {
//     use crate::prelude::*;
//     pub fn make_timestamp() -> Timestamp {
//         let now = std::time::SystemTime::now();
//         let since_epoch = now.duration_since(std::time::UNIX_EPOCH).unwrap();
//         Timestamp::from()
//     }
// }