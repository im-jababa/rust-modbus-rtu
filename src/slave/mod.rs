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

    pub fn analyze_packet(&self, packet: &[u8]) -> Result<RequestForm<'a>, PacketError> {
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

        match packet[1] {
            0x03 => {
                if self.holding_registers.is_empty() {
                    return Err(PacketError::Exeption(packet[1], Exception::IllegalFunction));
                }
                todo!()
            },
            0x04 => {
                if self.input_registers.is_empty() {
                    return Err(PacketError::Exeption(packet[1], Exception::IllegalFunction));
                }
                todo!()
            },
            0x06 => {
                if self.holding_registers.is_empty() {
                    return Err(PacketError::Exeption(packet[1], Exception::IllegalFunction));
                }
                todo!()
            },
            0x10 => {
                if self.holding_registers.is_empty() {
                    return Err(PacketError::Exeption(packet[1], Exception::IllegalFunction));
                }
                todo!()
            },
            #[cfg(feature="bypass")]
            0x45 => {
                todo!()
            },
            _ => return Err(PacketError::Exeption(packet[1], Exception::IllegalFunction)),
        }
    }
}
