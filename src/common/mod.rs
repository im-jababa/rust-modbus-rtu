//! Common features for both master device and slave device
//! 

pub mod crc;

mod baudrate;
pub use baudrate::Baudrate;

pub mod exception;
pub use exception::Exception;

pub mod packet_error;
pub use packet_error::PacketError;
