/// Represents an outgoing Modbus RTU request along with its metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request<'a> {
    modbus_id: u8,
    function: &'a crate::Function,
    timeout: std::time::Duration,
}


impl<'a> Request<'a> {
    /// Creates a new request for the specified Modbus slave, function, and timeout.
    ///
    /// ---
    /// # Examples
    /// ```rust
    /// use modbus_rtu::{Function, Request};
    ///
    /// let func = Function::ReadCoils { starting_address: 0x0000, quantity: 2 };
    /// let request = Request::new(0x01, &func, std::time::Duration::from_millis(200));
    ///
    /// assert_eq!(request.modbus_id(), 0x01);
    /// assert_eq!(request.timeout(), std::time::Duration::from_millis(200));
    /// ```
    /// 
    pub const fn new(modbus_id: u8, function: &'a crate::Function, timeout: std::time::Duration) -> Self {
        Self {
            modbus_id,
            function: function,
            timeout,
        }
    }

    /// Returns the Modbus slave identifier targeted by this request.
    pub const fn modbus_id(&self) -> u8 {
        self.modbus_id
    }

    /// Updates the Modbus slave identifier targeted by this request.
    pub fn set_modbus_id(&mut self, modbus_id: u8) {
        self.modbus_id = modbus_id;
    }

    /// Returns the function payload that will be issued with this request.
    pub const fn function(&self) -> &crate::Function {
        self.function
    }

    /// Replaces the function payload associated with this request.
    pub fn set_function(&mut self, function: &'a crate::Function) {
        self.function = function;
    }

    /// Returns the timeout associated with this request.
    pub const fn timeout(&self) -> std::time::Duration {
        self.timeout
    }

    /// Updates the timeout associated with this request.
    pub fn set_timeout(&mut self, timeout: std::time::Duration) {
        self.timeout = timeout;
    }

    pub fn is_broadcasting(&self) -> bool {
        self.modbus_id() == 0
    }

    /// Serializes the request into a Modbus RTU frame containing the device id,
    /// function payload, and CRC footer.
    ///
    /// # Errors
    /// Returns [`ReqPacketError`](crate::error::ReqPacketError) if the inner
    /// function cannot be encoded within the 256-byte packet limit.
    ///
    /// # Examples
    /// ```rust
    /// use modbus_rtu::{Function, Request};
    ///
    /// let func = Function::WriteSingleRegister { address: 0x0010, value: 0xABCD };
    /// let request = Request::new(0x11, &func, std::time::Duration::from_millis(100));
    /// let frame = request.to_bytes().unwrap();
    ///
    /// assert_eq!(&frame[..], &[0x11, 0x06, 0x00, 0x10, 0xAB, 0xCD, 0x34, 0x3A]);
    /// ```
    /// 
    pub fn to_bytes(&self) -> Result<Box<[u8]>, crate::error::RequestPacketError> {
        use crate::FunctionKind::*;
        if self.is_broadcasting() 
        && [ReadCoils, ReadDiscreteInputs, ReadHoldingRegisters, ReadInputRegisters]
        .contains(&self.function().kind()) {
            return Err(crate::error::RequestPacketError::CannotBroadcast);
        }
        let mut buf: Vec<u8> = Vec::new();
        buf.push(self.modbus_id());
        let bytes = self.function().to_bytes()?;
        buf.extend_from_slice(&bytes);
        let crc_bytes = crate::crc::generate(&buf[0..buf.len()]);
        buf.extend_from_slice(&crc_bytes.to_le_bytes());
        Ok(buf.into_boxed_slice())
    }
}
