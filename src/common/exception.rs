/// Modbus RTU exception codes.
/// 
/// ---
/// # Supports
/// - [`u8`] -> [`Exception`]
/// - [`Exception`] -> [`u8`]
/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Exception {
    /// An undefined exception code not covered by this crate.
    Undefined(u8),

    /// The function code received is not supported by the device or is invalid in the current state.
    IllegalFunction = 0x01,

    /// The requested address range is invalid for the device.
    IllegalDataAddress = 0x02,

    /// A value in the request is not valid or does not match the expected structure.
    IllegalDataValue = 0x03,

    /// An unrecoverable device error occurred during processing.
    DeviceFailure = 0x04,

    /// The request was accepted but requires a long time to complete. Prevents master timeout.
    Acknowledge = 0x05,

    /// The device is busy processing a long-duration command. Try again later.
    DeviceBusy = 0x06,

    /// 
    MemoryParityError = 0x08,

    /// The gateway could not establish a communication path. Check configuration or load.
    GatewayPathUnavailable = 0x0A,

    /// The gateway received no response from the target device.
    GatewayTargetDeviceFailedToRespond = 0x0B,
}


impl Exception {
    /// Returns `true` if the exception is [`Exception::Acknowledge`].
    ///
    /// ---
    /// # Returns
    /// `true` if the exception is [`Exception::Acknowledge`], otherwise `false`.
    /// 
    pub fn is_ack(&self) -> bool {
        matches!(self, Exception::Acknowledge)
    }
}


// u8 -> Exception
impl From<u8> for Exception {
    fn from(value: u8) -> Self {
        match value {
            0x01 => Exception::IllegalFunction,
            0x02 => Exception::IllegalDataAddress,
            0x03 => Exception::IllegalDataValue,
            0x04 => Exception::DeviceFailure,
            0x05 => Exception::Acknowledge,
            0x06 => Exception::DeviceBusy,
            0x08 => Exception::MemoryParityError,
            0x0A => Exception::GatewayPathUnavailable,
            0x0B => Exception::GatewayTargetDeviceFailedToRespond,
            _ => Exception::Undefined(value),
        }
    }
}


// Exception -> u8
impl From<Exception> for u8 {
    fn from(value: Exception) -> Self {
        match value {
            Exception::Undefined(c) => c,
            Exception::IllegalFunction => 0x01,
            Exception::IllegalDataAddress => 0x02,
            Exception::IllegalDataValue => 0x03,
            Exception::DeviceFailure => 0x04,
            Exception::Acknowledge => 0x05,
            Exception::DeviceBusy => 0x06,
            Exception::MemoryParityError => 0x08,
            Exception::GatewayPathUnavailable => 0x0A,
            Exception::GatewayTargetDeviceFailedToRespond => 0x0B,
        }
    }
}


// &Exception -> u8
impl From<&Exception> for u8 {
    fn from(value: &Exception) -> Self {
        match value {
            Exception::Undefined(c) => *c,
            Exception::IllegalFunction => 0x01,
            Exception::IllegalDataAddress => 0x02,
            Exception::IllegalDataValue => 0x03,
            Exception::DeviceFailure => 0x04,
            Exception::Acknowledge => 0x05,
            Exception::DeviceBusy => 0x06,
            Exception::MemoryParityError => 0x08,
            Exception::GatewayPathUnavailable => 0x0A,
            Exception::GatewayTargetDeviceFailedToRespond => 0x0B,
        }
    }
}


// Display
#[cfg(not(feature = "no_std"))]
impl std::fmt::Display for Exception {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use Exception::*;
        match self {
            Undefined(c) => write!(f, "Undefined exception code 0x{c:02}({c})"),
            IllegalFunction => write!(f, "Illegal function"),
            IllegalDataAddress => write!(f, "Illegal data address"),
            IllegalDataValue => write!(f, "Illegal data value"),
            DeviceFailure => write!(f, "Device failure"),
            Acknowledge => write!(f, "Acknowledge"),
            DeviceBusy => write!(f, "Device busy"),
            MemoryParityError => write!(f, "Memory parity error"),
            GatewayPathUnavailable => write!(f, "Gateway path unavailable"),
            GatewayTargetDeviceFailedToRespond => write!(f, "Gateway target device failed to respond"),
        }
    }
}
