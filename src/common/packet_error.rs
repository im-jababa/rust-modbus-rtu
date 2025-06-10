use crate::Exception;


/// Error types for received packet while analyzing it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketError {
    /// Packet is too short to start analyze (before checking CRC validation)
    TooShort(usize),

    /// Given CRC bytes are mismatched with the expected one
    CrcMismatch { expected: u16, received: u16 },

    /// Packet is from unexpected ID
    NotMyId(u8),

    /// Device reported exception code
    Exeption(Exception),

    /// Unexpected packet format (Master side only)
    Invalid(&'static str),
}
