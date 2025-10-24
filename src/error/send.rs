#[derive(Debug)]
pub enum SendError {
    RequestPacket(super::RequestPacketError),
    ResponsePacket(super::ResponsePacketError),
    Io(std::io::Error),
}

impl std::fmt::Display for SendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SendError::RequestPacket(request_packet_error) => request_packet_error.to_string(),
                SendError::ResponsePacket(response_packet_error) =>
                    response_packet_error.to_string(),
                SendError::Io(error) => error.to_string(),
            }
        )
    }
}

impl std::error::Error for SendError {}

impl From<super::RequestPacketError> for SendError {
    fn from(value: super::RequestPacketError) -> Self {
        SendError::RequestPacket(value)
    }
}

impl From<super::ResponsePacketError> for SendError {
    fn from(value: super::ResponsePacketError) -> Self {
        SendError::ResponsePacket(value)
    }
}

impl From<std::io::Error> for SendError {
    fn from(value: std::io::Error) -> Self {
        SendError::Io(value)
    }
}
