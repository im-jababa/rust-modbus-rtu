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

    /// The gateway could not establish a communication path. Check configuration or load.
    GatewayPathUnavailable = 0x0A,

    /// The gateway received no response from the target device.
    GatewayTargetDeviceFailedToRespond = 0x0B,
}


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
            Exception::GatewayPathUnavailable => 0x0A,
            Exception::GatewayTargetDeviceFailedToRespond => 0x0B,
        }
    }
}


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
            Exception::GatewayPathUnavailable => 0x0A,
            Exception::GatewayTargetDeviceFailedToRespond => 0x0B,
        }
    }
}


#[test]
fn test() {
    let code: u8 = Exception::IllegalFunction.into();
    assert_eq!(code, 0x01);

    let code: u8 = Exception::Undefined(0x16).into();
    assert_eq!(code, 0x16);
}
