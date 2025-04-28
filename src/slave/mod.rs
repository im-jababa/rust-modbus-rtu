mod data_model;     pub use data_model::{DataModel, DataStructure};

use crate::common::{crc, exception::Exception, PacketError, RequestForm};


/// Modbus Slave
#[derive(Debug)]
pub struct ModbusSlave<const L1: usize, const L2: usize> {
    /// The Modbus slave ID.
    ///
    /// Valid Modbus IDs range from `1` to `247`. This crate also supports reserved IDs from `248` to `255`.
    /// If the ID is set to `0`, the device will listen to all Modbus IDs and never respond.
    ///
    modbus_id: u8,

    /// Holding registers.
    ///
    /// Read-write registers that can be accessed and modified by the Modbus master.
    /// 
    holding_registers: DataModel<L1, u16>,

    /// Input registers.
    ///
    /// Read-only registers that can be accessed by the Modbus master.
    /// 
    input_registers: DataModel<L2, u16>,
}


impl<'a, const L1: usize, const L2: usize> ModbusSlave<L1, L2> {
    pub fn new(modbus_id: u8, holding_registers: DataModel<L1, u16>, input_registers: DataModel<L2, u16>) -> ModbusSlave<L1, L2> {
        ModbusSlave { modbus_id, holding_registers, input_registers }
    }

    pub fn get_modbus_id(&self) -> u8 {
        self.modbus_id
    }

    pub fn set_modbus_id(&mut self, modbus_id: u8) {
        self.modbus_id = modbus_id;
    }

    pub fn get_holding_registers(&self) -> &DataModel<L1, u16> {
        &self.holding_registers
    }

    pub fn get_holding_registers_mut(&mut self) -> &mut DataModel<L1, u16> {
        &mut self.holding_registers
    }

    pub fn get_input_registers(&self) -> &DataModel<L2, u16> {
        &self.input_registers
    }

    pub fn get_input_registers_mut(&mut self) -> &mut DataModel<L2, u16> {
        &mut self.input_registers
    }

    pub fn analyze_packet(&self, packet: &[u8], word_buffer: &'a mut [u16]) -> Result<RequestForm<'a>, PacketError> {
        let len = packet.len();

        // Packet too short
        if len < 4 {
            return Err(PacketError::TooShort(len));
        }

        // Validate CRC bytes
        crc::validate(packet)?;

        // Check modbus ID
        if (self.modbus_id != 0x00) && (self.modbus_id != packet[0]) {
            return Err(PacketError::NotMyId(packet[0]));
        }

        let fc  = packet[1];
        match fc {
            // Process read holding registers
            0x03 => {
                // If the holding registers have zero capacity, the request is treated as not supported.
                if self.holding_registers.is_empty() {
                    return Err(PacketError::Exeption(fc, Exception::IllegalFunction));
                }

                // Required length
                if len < 8 {
                    return Err(PacketError::Exeption(fc, Exception::IllegalDataValue));
                }

                let start_register: u16 = ((packet[2] as u16) << 8) | (packet[3] as u16);
                let registers_count: u16 = ((packet[4] as u16) << 8) | (packet[5] as u16);

                // Register address overflowed
                let end_register: u16 = match start_register.checked_add(registers_count) {
                    Some(v) => v,
                    None => return Err(PacketError::Exeption(fc, Exception::IllegalDataAddress)),
                };

                // All register addresses must be valid
                for adr in start_register..=end_register {
                    if let None = self.holding_registers.find_index(adr) {
                        return Err(PacketError::Exeption(fc, Exception::IllegalDataAddress));
                    }
                }

                Ok(RequestForm::ReadHoldingRegisters { start_register, registers_count })
            },

            // Process read input registers
            0x04 => {
                // If the input registers have zero capacity, the request is treated as not supported.
                if self.input_registers.is_empty() {
                    return Err(PacketError::Exeption(fc, Exception::IllegalFunction));
                }

                // Required length
                if len < 8 {
                    return Err(PacketError::Exeption(fc, Exception::IllegalDataValue));
                }
                
                let start_register: u16 = ((packet[2] as u16) << 8) | (packet[3] as u16);
                let registers_count: u16 = ((packet[4] as u16) << 8) | (packet[5] as u16);

                // Register address overflowed
                let end_register: u16 = match start_register.checked_add(registers_count) {
                    Some(v) => v,
                    None => return Err(PacketError::Exeption(fc, Exception::IllegalDataAddress)),
                };

                // All register addresses must be valid
                for adr in start_register..=end_register {
                    if let None = self.input_registers.find_index(adr) {
                        return Err(PacketError::Exeption(fc, Exception::IllegalDataAddress));
                    }
                }

                Ok(RequestForm::ReadInputRegisters { start_register, registers_count })
            },

            // Process write single register
            0x06 => {
                // If the holding registers have zero capacity, the request is treated as not supported.
                if self.holding_registers.is_empty() {
                    return Err(PacketError::Exeption(fc, Exception::IllegalFunction));
                }

                // Required length
                if len < 8 {
                    return Err(PacketError::Exeption(fc, Exception::IllegalDataValue));
                }
                
                let register_address: u16 = ((packet[2] as u16) << 8) | (packet[3] as u16);
                let data_to_write: u16 = ((packet[4] as u16) << 8) | (packet[5] as u16);

                // Register address must be valid
                if let None = self.holding_registers.find_index(register_address) {
                    return Err(PacketError::Exeption(fc, Exception::IllegalDataAddress));
                }

                Ok(RequestForm::WriteSingleRegister { register_address, data_to_write })
            },

            // Process write multiple registers
            0x10 => {
                // If the holding registers have zero capacity, the request is treated as not supported.
                if self.holding_registers.is_empty() {
                    return Err(PacketError::Exeption(fc, Exception::IllegalFunction));
                }
                
                // Minimum required length
                if len < 9 {
                    return Err(PacketError::Exeption(fc, Exception::IllegalDataValue));
                }

                let start_register: u16 = ((packet[2] as u16) << 8) | (packet[3] as u16);
                let registers_count: u16 = ((packet[4] as u16) << 8) | (packet[5] as u16);
                let bytes_count: u8 = packet[6];

                // Required length
                if len < 9 + bytes_count as usize {
                    return Err(PacketError::Exeption(fc, Exception::IllegalDataValue));
                }

                // Bytes count must be double of the registers count
                if bytes_count as u16 != registers_count.saturating_mul(2) {
                    return Err(PacketError::Exeption(fc, Exception::IllegalDataValue));
                }

                // Register address overflowed
                let end_register: u16 = match start_register.checked_add(registers_count) {
                    Some(v) => v,
                    None => return Err(PacketError::Exeption(fc, Exception::IllegalDataAddress)),
                };

                // All register addresses must be valid
                for adr in start_register..=end_register {
                    if let None = self.holding_registers.find_index(adr) {
                        return Err(PacketError::Exeption(fc, Exception::IllegalDataAddress));
                    }
                }

                // All data to write
                for i in 0..registers_count as usize {
                    word_buffer[i] = ((packet[7 + (i * 2) + 0] as u16) << 8) | (packet[7 + (i * 2) + 1] as u16);
                }

                Ok(RequestForm::WriteMultipleRegisters { start_register, data_to_write: &word_buffer[..registers_count as usize] })
            },

            // Process bypass packet
            #[cfg(feature="bypass")]
            0x45 => {
                todo!()
            },
            _ => return Err(PacketError::Exeption(fc, Exception::IllegalFunction)),
        }
    }

    pub fn build_exception_response_packet(&self, fc: u8, exception: Exception) -> [u8; 5] {
        let mut result: [u8; 5] = [
            self.modbus_id,
            fc | 0x80,
            exception.into(),
            0x00,
            0x00,
        ];

        let crc_bytes = crc::gen_bytes(&result[..3]);
        result[3..5].copy_from_slice(&crc_bytes);

        result
    }
}
