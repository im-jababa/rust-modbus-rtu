/// Modbus RTU request packet format
#[derive(Debug)]
pub enum RequestForm<'a> {
    /// Request format for reading multiple Holding Registers
    /// 
    /// - `start_register`: The starting register address
    /// - `registers_count`: The number of registers to read
    /// 
    ReadHoldingRegisters { start_register: u16, registers_count: u16 },

    /// Request format for reading multiple Input Registers
    /// 
    /// - `start_register`: The starting register address
    /// - `registers_count`: The number of registers to read
    /// 
    ReadInputRegisters { start_register: u16, registers_count: u16 },

    /// Request format for writing a single Holding Register
    /// 
    /// - `register_address`: The register address to write to
    /// - `data_to_write`: The data value to write
    /// 
    WriteSingleRegister { register_address: u16, data_to_write: u16 },

    /// Request format for writing multiple Holding Registers
    /// 
    /// - `start_register`: The starting register address
    /// - `datas_to_wirte`: Slice of data values to write to consecutive registers
    /// 
    WriteMultipleRegisters { start_register: u16, datas_to_write: &'a [u16] },

    /// Request format for bypassing a packet to a downstream device
    #[cfg(feature="bypass")]
    BypassRequest,
}


impl<'a> RequestForm<'a> {
    /// Retrieves the Modbus function code corresponding to the request form variant.
    ///
    /// ---
    /// # Returns
    /// A `u8` representing the Modbus function code of the request.
    ///
    /// ---
    /// # Examples
    /// ```
    /// use modbus_rtu::common::RequestForm;
    ///
    /// let form = RequestForm::ReadHoldingRegisters {
    ///     start_register: 0x0000,
    ///     registers_count: 2,
    /// };
    ///
    /// assert_eq!(form.get_function_code(), 0x03);
    /// ```
    /// 
    pub fn get_function_code(&self) -> u8 {
        match self {
            RequestForm::ReadHoldingRegisters { .. } => 0x03,
            RequestForm::ReadInputRegisters { .. } => 0x04,
            RequestForm::WriteSingleRegister { .. } => 0x06,
            RequestForm::WriteMultipleRegisters { .. } => 0x10,
            #[cfg(feature="bypass")]
            RequestForm::BypassRequest => 0x45,
        }
    }
}
