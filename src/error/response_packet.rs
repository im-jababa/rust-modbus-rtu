pub enum ResponsePacketError {
    TooShort(usize),
    CRCMismatch { expected: u16, received: u16 },
    UnexpectedResponder(u8),
    InvalidFormat,
}
