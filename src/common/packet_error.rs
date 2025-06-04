use super::exception::Exception;


/// Error types for received packet while analyzing it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketError {
    /// Packet is too short to analyze
    TooShort(usize),

    /// Given CRC bytes are mismatched with the expected one
    CrcMismatch { expected: [u8; 2], found: [u8; 2] },

    /// Packet is from unexpected ID
    NotMyId(u8),

    /// Device reported exception code
    Exeption(u8, Exception),

    /// Unexpected packet format
    Invalid(&'static str),
}
