/// Enumerates the Modbus application exceptions returned by a slave device,
/// including a catch-all for codes not defined by the specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Exception {
    /// Exception code not defined by this crate; preserves the raw value for
    /// diagnostics.
    Undefined(u8) = 0x00,

    /// Exception code `0x01`: the function is not supported or the device state
    /// does not allow the requested operation (for example an unconfigured unit
    /// asked to return registers).
    IllegalFunction = 0x01,

    /// Exception code `0x02`: the combination of starting address and length
    /// extends beyond the valid data range implemented by the device.
    IllegalDataAddress = 0x02,

    /// Exception code `0x03`: a value in the request payload is not acceptable
    /// to the device, or the payload structure is inconsistent (for example an
    /// incorrect implied length).
    IllegalDataValue = 0x03,

    /// Exception code `0x04`: an unrecoverable fault occurred while the device
    /// attempted the requested action.
    DeviceFailure = 0x04,

    /// Exception code `0x05`: the device accepted the request but needs a long
    /// interval to finish; the client should poll later (e.g., Poll Program
    /// Complete).
    Acknowledge = 0x05,

    /// Exception code `0x06`: the device is busy processing a long-duration
    /// command and cannot handle the new request yet; the client should retry
    /// later.
    DeviceBusy = 0x06,

    /// Exception code `0x08`: while accessing extended file records (function
    /// codes 20/21, reference type 6) the device detected a memory parity error.
    MemoryParityError = 0x08,

    /// Exception code `0x0A`: a gateway could not allocate an internal path
    /// between the input and output ports, often due to misconfiguration or
    /// overload.
    GatewayPathUnavailable = 0x0A,

    /// Exception code `0x0B`: a gateway forwarded the request but received no
    /// response from the target device, which may be offline or unreachable.
    GatewayTargetDeviceFailedToRespond = 0x0B,
}


impl Exception {
    /// Returns the Modbus exception code associated with this variant.
    ///
    /// # Examples
    /// ```rust
    /// use modbus_rtu::Exception;
    ///
    /// assert_eq!(Exception::Undefined(0x7F).as_code(), 0x7F);
    /// assert_eq!(Exception::IllegalDataAddress.as_code(), 0x02);
    /// assert_eq!(Exception::DeviceBusy.as_code(), 0x06);
    /// ```
    /// 
    pub const fn as_code(&self) -> u8 {
        match self {
            Exception::Undefined(code) => *code,
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

    /// Converts a Modbus exception code into its corresponding [`Exception`]
    /// variant. Undefined codes are wrapped in [`Exception::Undefined`].
    ///
    /// # Examples
    /// ```rust
    /// use modbus_rtu::Exception;
    ///
    /// assert_eq!(Exception::from_code(0x05), Exception::Acknowledge);
    /// assert_eq!(Exception::from_code(0xFF), Exception::Undefined(0xFF));
    /// ```
    /// 
    pub fn from_code(code: u8) -> Self {
        match code {
            0x01 => Self::IllegalFunction,
            0x02 => Self::IllegalDataAddress,
            0x03 => Self::IllegalDataValue,
            0x04 => Self::DeviceFailure,
            0x05 => Self::Acknowledge,
            0x06 => Self::DeviceBusy,
            0x08 => Self::MemoryParityError,
            0x0A => Self::GatewayPathUnavailable,
            0x0B => Self::GatewayTargetDeviceFailedToRespond,
            code => Self::Undefined(code),
        }
    }
}
