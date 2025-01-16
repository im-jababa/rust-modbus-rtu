pub enum ReqPacket {
    ReadHoldingRegisters{slave: u8, base_register_address: u16, number_of_registers: u16},
    ReadInputRegisters{slave: u8, base_register_address: u16, number_of_registers: u16},
    WriteSingleHoldingRegister{slave: u8, register_address: u16, data: u16},
}


impl ReqPacket {
    pub fn execute() {
        
    }
}