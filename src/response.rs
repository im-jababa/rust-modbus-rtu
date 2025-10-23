#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Response {
    Status(Box<[bool]>),
    Value(Box<u16>),
    Success,
    Exception(crate::Exception),
}


impl Response {
    pub fn from_bytes(request: &crate::Request, bytes: &[u8]) -> Result<Self, crate::error::ResponsePacketError> {
        // length check
        let len = bytes.len();
        if len < 5 {
            return Err(crate::error::ResponsePacketError::TooShort(len));
        }

        // crc check
        crate::crc::validate(&bytes[0..(len - 2)])?;

        // exception check
        let function_code = bytes[1];
        if function_code & 0x80 != 0 {
            let code = bytes[2];
            return Ok(Self::Exception(crate::Exception::from_code(code)));
        }

        let function_kind = match crate::FunctionKind::from_code(function_code) {
            Some(kind) => kind,
            None => return Err(crate::error::ResponsePacketError::InvalidFormat),
        };
        match function_kind {
            crate::FunctionKind::ReadCoils => todo!(),
            crate::FunctionKind::ReadDiscreteInputs => todo!(),
            crate::FunctionKind::ReadHoldingRegisters => todo!(),
            crate::FunctionKind::ReadInputRegisters => todo!(),
            crate::FunctionKind::WriteSingleCoil => todo!(),
            crate::FunctionKind::WriteSingleRegister => todo!(),
            crate::FunctionKind::WriteMultipleCoils => todo!(),
            crate::FunctionKind::WriteMultipleRegisters => todo!(),
        }

        todo!()
    }
}
