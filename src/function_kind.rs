/// ## FunctionKind
/// 
/// `FunctionKind` represents the function codes defined by the Modbus RTU standard protocol.
/// Functions not listed here are not supported.
/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FunctionKind {
    /// Read Coils `(0x01)`
    ReadCoils = 0x01,

    /// Read Discrete Inputs `(0x02)`
    ReadDiscreteInputs = 0x02,

    /// Read Holding Registers `(0x03)`
    ReadHoldingRegisters = 0x03,

    /// Read Input Registers `(0x04)`
    ReadInputRegisters = 0x04,

    /// Write Single Coil `(0x05)`
    WriteSingleCoil = 0x05,

    /// Write Single Register `(0x06)`
    WriteSingleRegister = 0x06,

    /// Write Multiple Coils `(0x0F)`
    WriteMultipleCoils = 0x0F,

    /// Write Multiple Registers `(0x10)`
    WriteMultipleRegisters = 0x10,
}


impl FunctionKind {
    /// Returns the Modbus RTU function code represented by this [`FunctionKind`].
    ///
    /// # Examples
    /// ```rust
    /// use modbus_rtu::FunctionKind;
    ///
    /// let code = FunctionKind::ReadHoldingRegisters.as_code();
    /// assert_eq!(code, 0x03);
    /// ```
    /// 
    pub fn as_code(&self) -> u8 {
        *self as u8
    }

    /// Converts a Modbus RTU function code into its [`FunctionKind`] counterpart.
    ///
    /// Returns [`Some`] when the code is supported by this crate; otherwise
    /// returns [`None`].
    ///
    /// # Examples
    /// ```rust
    /// use modbus_rtu::FunctionKind;
    ///
    /// let kind = FunctionKind::from_code(0x04);
    /// assert_eq!(kind, Some(FunctionKind::ReadInputRegisters));
    ///
    /// let unsupported = FunctionKind::from_code(0x7F);
    /// assert_eq!(unsupported, None);
    /// ```
    /// 
    pub fn from_code(code: u8) -> Option<Self> {
        match code {
            0x01 => Some(Self::ReadCoils),
            0x02 => Some(Self::ReadDiscreteInputs),
            0x03 => Some(Self::ReadHoldingRegisters),
            0x04 => Some(Self::ReadInputRegisters),
            0x05 => Some(Self::WriteSingleCoil),
            0x06 => Some(Self::WriteSingleRegister),
            0x0F => Some(Self::WriteMultipleCoils),
            0x10 => Some(Self::WriteMultipleRegisters),
            _    => None,
        }
    }
}


impl std::fmt::Display for FunctionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::ReadCoils => "Read Coils",
            Self::ReadDiscreteInputs => "Read Discrete Inputs",
            Self::ReadHoldingRegisters => "Read Holding Registers",
            Self::ReadInputRegisters => "Read Input Registers",
            Self::WriteSingleCoil => "Write Single Coil",
            Self::WriteSingleRegister => "Write Single Register",
            Self::WriteMultipleCoils => "Write Multiple Coils",
            Self::WriteMultipleRegisters => "Write Multiple Registers",
        })
    }
}
