/// ## Function error
/// 
/// This error is raised when the function tries to produce a request or response
/// packet that exceeds the Modbus RTU protocol's maximum packet length of 256
/// bytes.
/// 
/// Requests that attempt to read or write too many values at once will exceed
/// the 256-byte limit of the request or response packet.
/// 
/// By default, this crate uses a 256-byte buffer.
/// If you want to avoid this error or intentionally allow packets to exceed the
/// limit, enable the feature in `Cargo.toml` as shown below.
/// 
/// ```toml
/// [dependencies]
/// modbus-rtu = { version = "1.0", features = ["unlimited_packet_size"] }
/// ```
/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct FunctionError;


impl std::fmt::Display for FunctionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Function packet length exceeds the Modbus RTU 256-byte limit; enable the `unlimited_packet_size` feature to allow larger packets."
        )
    }
}


impl std::error::Error for FunctionError {}
