pub mod crc;
pub mod exception;
mod request;        pub use request::Request;
mod baudrate;       pub use baudrate::Baudrate;
mod request_form;   pub use request_form::RequestForm;
mod packet_error;   pub use packet_error::PacketError;

#[cfg(feature="bypass")]
pub use request::BypassRequest;

#[cfg(feature="bypass")]
pub use request_form::BypassRequestForm;
