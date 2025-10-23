/// ## Function error
/// 
/// This error is raised when the function tries to produce a request packet
/// that exceeds the Modbus RTU protocol's maximum packet length of 256 bytes.
/// 
/// Requests that attempt to write too many values at once will exceed
/// the 256-byte limit of the request packet.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FunctionError;


impl std::fmt::Display for FunctionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Function packet length exceeds the Modbus RTU 256-byte limit."
        )
    }
}


impl std::error::Error for FunctionError {}
