use super::exception::Exception;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PacketError {
    TooShort(usize),
    CrcMismatch { expected: [u8; 2], found: [u8; 2] },
    NotMyId(u8),
    Exeption(Exception),
}


#[cfg(feature = "std")]
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {

        }
    }
}


#[cfg(feature = "std")]
impl std::error::Error for Error {}
