use crate::utils::{crc16_modbus, vec_bool_to_vec_u8};


pub const BROADCAST: u8 = 0;


/// ### Request
/// Modbus RTU request packet variants.
#[derive(Debug, Clone)]
pub enum Request {
    /// ### ReadCoils (`0x01`)
    /// Request to read multiple coils. A coil is a single-bit memory that the master can **read and write**.
    /// - `slave`: Address of the target device. It must not be zero.
    /// - `base_address`: Base coil address to start reading. `base_address + quantity - 1` must not exceed `0xFFFF`.
    /// - `quantity`: Number of coils to read. It must be **less than or equal to** `255` due to the response byte counter being 8-bit.
    ReadCoils {slave: u8, base_address: u16, quantity: u16},

    /// ### ReadDiscreteInputs (`0x02`)
    /// Request to read multiple discrete inputs. A discrete input is a single-bit memory that the master can **only read**.
    /// - `slave`: Address of the target device. It must not be zero.
    /// - `base_address`: Base discrete input address to start reading. `base_address + quantity - 1` must not exceed `0xFFFF`.
    /// - `quantity`: Number of discrete inputs to read. It must be **less than or equal to** `255` due to the response byte counter being 8-bit.
    ReadDiscreteInputs {slave: u8, base_address: u16, quantity: u16},

    /// ### ReadHoldingRegisters (`0x03`)
    /// Request to read multiple holding registers. A holding register is a 16-bit memory that the master can **read and write**.
    /// - `slave`: Address of the target device. It must not be zero.
    /// - `base_address`: Base holding register address to start reading. `base_address + quantity - 1` must not exceed `0xFFFF`.
    /// - `quantity`: Number of holding registers to read. It must be **less than or equal to** `125` due to the maximum packet size of `256`.
    ReadHoldingRegisters {slave: u8, base_address: u16, quantity: u16},

    /// ### ReadInputRegisters (`0x04`)
    /// Request to read multiple input registers. An input register is a 16-bit memory that the master can **only read**.
    /// - `slave`: Address of the target device. It must not be zero.
    /// - `base_address`: Base input register address to start reading. `base_address + quantity - 1` must not exceed `0xFFFF`.
    /// - `quantity`: Number of input registers to read. It must be **less than or equal to** `125` due to the maximum packet size of `256`.
    ReadInputRegisters {slave: u8, base_address: u16, quantity: u16},

    /// ### WriteSingleCoil (`0x05`)
    /// Request to write a single coil. A coil is a single-bit memory that the master can **read and write**.
    /// - `slave`: Address of the target device.
    /// - `address`: Coil address to write to.
    /// - `data`: Data to write. `true` represents `ON`, and `false` represents `OFF`.
    WriteSingleCoil {slave: u8, address: u16, data: bool},

    /// ### WriteSingleRegister (`0x06`)
    /// Request to write a single holding register. A holding register is a 16-bit memory that the master can **read and write**.
    /// - `slave`: Address of the target device.
    /// - `address`: Holding register address to write to.
    /// - `data`: Data to write.
    WriteSingleRegister {slave: u8, address: u16, data: u16},

    /// ### WriteMultipleCoils (`0x0F`)
    /// Request to write multiple coils. Coils are single-bit memory that the master can **read and write**.
    /// - `slave`: Address of the target device to write to.
    /// - `base_address`: Base coil address to start writing. `base_address + length_of_data - 1` must not exceed `0xFFFF`.
    /// - `data`: Data to write. The length of the data must be **less than or equal to** `1976` due to the maximum packet size of `256`.
    WriteMultipleCoils {slave: u8, base_address: u16, data: Vec<bool>},

    /// ### WriteMultipleRegisters (`0x10`)
    /// Request to write multiple holding registers. Holding registers are 16-bit memory that the master can **read and write**.
    /// - `slave`: Address of the target device.
    /// - `base_address`: Base holding register address to start writing. `base_address + length_of_data - 1` must not exceed `0xFFFF`.
    /// - `data`: Data to write. The length of the data must be **less than or equal to** `123` due to the maximum packet size of `256`.
    WriteMultipleRegisters {slave: u8, base_address: u16, data: Vec<u16>},

    /// ### MaskWriteRegister (`0x16`)
    /// Request to apply AND and OR masks to a single holding register. A holding register is a 16-bit memory that the master can **read and write**.
    /// - `slave`: Address of the target device.
    /// - `address`: Holding register address to apply masks to.
    /// - `and_mask`: AND mask to apply.
    /// - `or_mask`: OR mask to apply.
    MaskWriteRegister {slave: u8, address: u16, and_mask: u16, or_mask: u16},

    /// ### ReadWriteMultipleRegisters (`0x17`)
    /// Request to write to multiple holding registers and read from multiple holding registers.
    /// Holding registers are 16-bit memory that the master can **read and write**.
    /// - `slave`: Address of the target device. It must not be zero.
    /// - `read_base_address`: Base holding register address to start reading. `read_base_address + read_quantity - 1` must not exceed `0xFFFF`.
    /// - `read_quantity`: Number of holding registers to read. It must be **less than or equal to** `125` due to the maximum packet size of `256`.
    /// - `write_base_address`: Base holding register address to start writing. `write_base_address + length_of_data - 1` must not exceed `0xFFFF`.
    /// - `data`: Data to write. The length of the data must be **less than or equal to** `121` due to the maximum packet size of `256`.
    ReadWriteMultipleRegisters {slave: u8, read_base_address: u16, read_quantity: u16, write_base_address: u16, data: Vec<u16>},
}

impl Request {
    /// ### function_code
    /// Returns the function code that corresponds to the request.
    pub fn function_code(&self) -> u8 {
        match self {
            Request::ReadCoils { .. } => 0x01,
            Request::ReadDiscreteInputs { .. } => 0x02,
            Request::ReadHoldingRegisters { .. } => 0x03,
            Request::ReadInputRegisters { .. } => 0x04,
            Request::WriteSingleCoil { .. } => 0x05,
            Request::WriteSingleRegister { .. } => 0x06,
            Request::WriteMultipleCoils { .. } => 0x0F,
            Request::WriteMultipleRegisters { .. } => 0x10,
            Request::MaskWriteRegister { .. } => 0x16,
            Request::ReadWriteMultipleRegisters { .. } => 0x17,
        }
    }

    /// ### expect_len
    /// Return expect response packet size.
    /// **Note** that this size is in case of the slave return successfully. If slave returns expection code, you should expect for 5 bytes.
    pub fn expect_len(&self) -> usize {
        match self {
            Request::ReadCoils { quantity, .. } |
            Request::ReadDiscreteInputs { quantity, .. } => {
                5 + ((*quantity as usize + 7) / 8)
            },
            Request::ReadHoldingRegisters { quantity, .. } |
            Request::ReadInputRegisters { quantity, .. } => {
                5 + (*quantity as usize * 2)
            },
            Request::WriteSingleCoil { .. } |
            Request::WriteSingleRegister { .. } |
            Request::WriteMultipleCoils { .. } |
            Request::WriteMultipleRegisters { .. } => {
                8
            },
            Request::MaskWriteRegister { .. } => {
                10
            },
            Request::ReadWriteMultipleRegisters { read_quantity, .. } => {
                5 + (*read_quantity as usize * 2)
            },
        }
    }

    /// ### to_byte
    /// Generate actual packet to send
    pub fn to_bytes(&self) -> Result<Vec<u8>, RequestError> {
        match self {
            Request::ReadCoils { slave, base_address, quantity } |
            Request::ReadDiscreteInputs { slave, base_address, quantity } => {
                if *slave == BROADCAST {
                    return Err(RequestError::IllegalBroadcasting);
                }
                if *quantity > 255 {
                    return Err(RequestError::ReadQuantityTooBig);
                }
                if *quantity > 0 && base_address.checked_add(quantity - 1_u16).is_none() {
                    return Err(RequestError::MemoryAddressExceed);
                }
                Request::default_packet(*slave, self.function_code(), *base_address, *quantity)
            },

            Request::ReadHoldingRegisters { slave, base_address, quantity } |
            Request::ReadInputRegisters { slave, base_address, quantity } => {
                if *slave == BROADCAST {
                    return Err(RequestError::IllegalBroadcasting);
                }
                if *quantity > 125 {
                    return Err(RequestError::ReadQuantityTooBig);
                }
                if *quantity > 0 && base_address.checked_add(quantity - 1_u16).is_none() {
                    return Err(RequestError::MemoryAddressExceed);
                }
                Request::default_packet(*slave, self.function_code(), *base_address, *quantity)
            },

            Request::WriteSingleCoil { slave, address, data } => {
                Request::default_packet(*slave, 0x05, *address, if *data {0xFF00} else {0x0000})
            },
            Request::WriteSingleRegister { slave, address, data } => {
                Request::default_packet(*slave, 0x06, *address, *data)
            },
            Request::WriteMultipleCoils { slave, base_address, data } => {
                if data.len() > 0 && base_address.checked_add(data.len() as u16 - 1_u16).is_none() {
                    return Err(RequestError::MemoryAddressExceed);
                }
                let bytes = vec_bool_to_vec_u8(&data);
                if bytes.len() > 247 {
                    return Err(RequestError::WriteQuantityTooBig);
                }
                let mut packet: Vec<u8> = Vec::with_capacity(bytes.len() + 9);
                packet.push(*slave); // slave address
                packet.push(0x0F); // function code
                packet.push(base_address.to_be_bytes()[0]); // base address high byte
                packet.push(base_address.to_be_bytes()[1]); // base address low byte
                packet.push((data.len() as u16).to_be_bytes()[0]); // num of coils high byte
                packet.push((data.len() as u16).to_be_bytes()[1]); // num of coils low byte
                packet.push(bytes.len() as u8); // num of data bytes
                for byte in bytes {
                    packet.push(byte); // byte group by 8 data
                }
                let crc = crc16_modbus(&packet); // generate crc bytes
                packet.push(crc[0]); // crc low byte
                packet.push(crc[1]); // crc high byte
                Ok(packet)
            },
            Request::WriteMultipleRegisters { slave, base_address, data } => {
                if data.len() > 123 {
                    return Err(RequestError::WriteQuantityTooBig);
                }
                if data.len() > 0 && base_address.checked_add(data.len() as u16 - 1_u16).is_none() {
                    return Err(RequestError::MemoryAddressExceed);
                }
                let mut packet: Vec<u8> = Vec::with_capacity((data.len() * 2) + 9);
                packet.push(*slave); // slave address
                packet.push(0x10); // function code
                packet.push(base_address.to_be_bytes()[0]); // base address high byte
                packet.push(base_address.to_be_bytes()[1]); // base address low byte
                packet.push((data.len() as u16).to_be_bytes()[0]); // num of registers high byte
                packet.push((data.len() as u16).to_be_bytes()[1]); // num of registers low byte
                packet.push((data.len() * 2) as u8); // num of data bytes
                for byte in data {
                    packet.push(byte.to_be_bytes()[0]); // data high byte
                    packet.push(byte.to_be_bytes()[1]); // data low byte
                }
                let crc = crc16_modbus(&packet); // generate crc bytes
                packet.push(crc[0]); // crc low byte
                packet.push(crc[1]); // crc high byte
                Ok(packet)
            },
            Request::MaskWriteRegister { slave, address, and_mask, or_mask } => {
                let mut packet: Vec<u8> = Vec::with_capacity(10);
                packet.push(*slave); // slave address
                packet.push(0x16); // function code
                packet.push(address.to_be_bytes()[0]); // address high byte
                packet.push(address.to_be_bytes()[1]); // address low byte
                packet.push(and_mask.to_be_bytes()[0]); // AND mask high byte
                packet.push(and_mask.to_be_bytes()[1]); // AND mask low byte
                packet.push(or_mask.to_be_bytes()[0]); // OR mask high byte
                packet.push(or_mask.to_be_bytes()[1]); // OR mask low byte
                let crc = crc16_modbus(&packet); // generate crc bytes
                packet.push(crc[0]); // crc low byte
                packet.push(crc[1]); // crc high byte
                Ok(packet)
            },
            Request::ReadWriteMultipleRegisters { slave, read_base_address, read_quantity, write_base_address, data } => {
                if *slave == BROADCAST {
                    return Err(RequestError::IllegalBroadcasting);
                }
                if *read_quantity > 125 {
                    return Err(RequestError::ReadQuantityTooBig);
                }
                if data.len() > 121 {
                    return Err(RequestError::WriteQuantityTooBig);
                }
                if *read_quantity > 0 && read_base_address.checked_add(read_quantity - 1_u16).is_none() {
                    return Err(RequestError::MemoryAddressExceed);
                }
                if data.len() > 0 && write_base_address.checked_add(data.len() as u16 - 1_u16).is_none() {
                    return Err(RequestError::MemoryAddressExceed);
                }
                let mut packet: Vec<u8> = Vec::with_capacity((data.len() * 2) + 13);
                packet.push(*slave); // slave address
                packet.push(0x17); // function code
                packet.push(read_base_address.to_be_bytes()[0]); // base register address to read to high byte
                packet.push(read_base_address.to_be_bytes()[1]); // base register address to read to low byte
                packet.push(read_quantity.to_be_bytes()[0]); // num of registers to read to high byte
                packet.push(read_quantity.to_be_bytes()[1]); // num of registers to read to low byte
                packet.push(write_base_address.to_be_bytes()[0]); // base register address to write to high byte
                packet.push(write_base_address.to_be_bytes()[1]); // base register address to write to low byte
                packet.push((data.len() as u16).to_be_bytes()[0]); // num of registers to write to high byte
                packet.push((data.len() as u16).to_be_bytes()[1]); // num of registers to write to low byte
                packet.push((data.len() * 2) as u8); // num of data bytes to write
                for byte in data {
                    packet.push(byte.to_be_bytes()[0]); // data bytes to write high byte
                    packet.push(byte.to_be_bytes()[1]); // data bytes to write low byte
                }
                let crc = crc16_modbus(&packet); // generate crc bytes
                packet.push(crc[0]); // crc low byte
                packet.push(crc[1]); // crc high byte
                Ok(packet)
            },
        }
    }

    fn default_packet(slave: u8, function_code: u8, address: u16, data: u16) -> Result<Vec<u8>, RequestError> {
        let mut packet: Vec<u8> = Vec::with_capacity(8);
        packet.push(slave); // slave address
        packet.push(function_code); // function code
        packet.push(address.to_be_bytes()[0]); // base address high byte
        packet.push(address.to_be_bytes()[1]); // base address low byte
        packet.push(data.to_be_bytes()[0]); // quantity high byte
        packet.push(data.to_be_bytes()[1]); // quantity low byte
        let crc = crc16_modbus(&packet); // generate crc bytes
        packet.push(crc[0]); // crc low byte
        packet.push(crc[1]); // crc high byte
        Ok(packet)
    }
}


/// ### RequestError
/// Errors that may occur when generating a Modbus RTU request packet.
#[derive(Debug)]
pub enum RequestError {
    /// ### IllegalBroadcasting
    /// This error occurs when attempting to broadcast with functions that are not write-only.
    IllegalBroadcasting,

    /// ### MemoryAddressExceed
    /// This error occurs when attempting to access addresses beyond `0xFFFF`.
    MemoryAddressExceed,

    /// ### ReadQuantityTooBig
    /// This error occurs when attempting to read more memory than allowed from the slave at once.
    ReadQuantityTooBig,

    /// ### WriteQuantityTooBig
    /// This error occurs when attempting to write more memory than allowed to the slave at once.
    WriteQuantityTooBig,
}

impl std::fmt::Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestError::IllegalBroadcasting => write!(f, "Broadcasting is not allowed to this function code."),
            RequestError::MemoryAddressExceed => write!(f, "Attempting to access beyond 0xFFFF has been detected."),
            RequestError::ReadQuantityTooBig => write!(f, "Attempting to read more memories than allowed at once."),
            RequestError::WriteQuantityTooBig => write!(f, "Attempting to write more memories than allowed at once."),
        }
    }
}


/// ### Response
#[derive(Debug, Clone)]
pub struct Response {
    /// ### buffer
    /// Buffer to recieve response packets from the slave.
    buffer: Vec<u8>,

    /// ### request
    /// The request for this response.
    request: Request,
}

impl Response {
    /// ### from_request
    /// Create a new corresponding Response type from the request.
    pub fn from_request(request: Request) -> Option<Response> {
        if request.to_bytes().is_err() {
            return None;
        }
        Some(Response { buffer: Vec::with_capacity(request.expect_len()), request })
    }

    pub fn analyze(&self) {

    }
}

pub enum ResponseState {
    Empty,
    Receiving(usize),
    
}