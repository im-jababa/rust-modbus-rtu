use super::{crc, RequestForm, BypassRequestForm};


#[derive(Debug)]
pub struct Request<'a> {
    modbus_id: u8,
    form: &'a RequestForm<'a>,
}


impl<'a> Request<'a> {
    /// Creates a new Modbus RTU request instance.
    ///
    /// ---
    /// # Arguments
    /// - `modbus_id`: The Modbus slave ID
    /// - `form`: A reference to the request form
    ///
    /// ---
    /// # Returns
    /// A new instance of `Request` containing the specified Modbus ID and form
    /// 
    /// ---
    /// # Examples
    /// ```
    /// use modbus_rtu::common::{Request, RequestForm};
    /// 
    /// let read_value_form = RequestForm::ReadInputRegisters {
    ///     start_register: 0x0000,
    ///     registers_count: 12,
    /// };
    /// 
    /// let request = Request::new(0x01, &read_value_form);
    /// ```
    ///
    pub fn new(modbus_id: u8, form: &'a RequestForm) -> Request<'a> {
        Request { modbus_id, form }
    }

    /// Writes a Modbus RTU request packet into the provided buffer and returns the corresponding slice.
    ///
    /// ---
    /// # Arguments
    /// - `buffer`: The buffer into which the packet will be written
    ///
    /// ---
    /// # Returns
    /// A slice representing the constructed Modbus RTU request packet
    ///
    /// ---
    /// # Examples
    /// ```
    /// use modbus_rtu::common::{Request, RequestForm};
    ///
    /// let write_datas_form = RequestForm::WriteMultipleRegisters {
    ///     start_register: 0x0001,
    ///     datas_to_write: &[0x1234, 0x5678],
    /// };
    ///
    /// let request = Request::new(0x01, &write_datas_form);
    ///
    /// let mut buffer: [u8; 256] = [0; 256];
    /// let packet = request.to_packet(&mut buffer);
    ///
    /// assert_eq!(packet, &[0x01, 0x10, 0x00, 0x01, 0x00, 0x02, 0x04, 0x12, 0x34, 0x56, 0x78, 0x49, 0x57]);
    /// ```
    /// 
    pub fn to_packet(&self, buffer: &'a mut [u8; 256]) -> &'a [u8] {
        // write modbus id
        buffer[0] = self.modbus_id;

        // write function code
        buffer[1] = self.form.get_function_code();

        // write data bytes
        let len: usize = match &self.form {
            RequestForm::ReadHoldingRegisters { start_register, registers_count } |
            RequestForm::ReadInputRegisters { start_register, registers_count } => {
                // write start register address
                buffer[2..4].copy_from_slice(&start_register.to_be_bytes());

                // write registers count
                buffer[4..6].copy_from_slice(&registers_count.to_be_bytes());

                // packet length without CRC bytes
                6
            },
            RequestForm::WriteSingleRegister { register_address, data_to_write } => {
                // write register address
                buffer[2..4].copy_from_slice(&register_address.to_be_bytes());

                // write data to write
                buffer[4..6].copy_from_slice(&data_to_write.to_be_bytes());

                // packet length without CRC bytes
                6
            },
            RequestForm::WriteMultipleRegisters { start_register, data_to_write } => {
                // write start register address
                buffer[2..4].copy_from_slice(&start_register.to_be_bytes());

                // write registers count
                let registers_count: u16 = data_to_write.len() as u16;
                buffer[4..6].copy_from_slice(&registers_count.to_be_bytes());

                // write bytes count
                buffer[6] = (registers_count * 2) as u8;

                // write datas to write
                for i in 0..registers_count as usize {
                    buffer[(7 + (i * 2))..=(8 + (i * 2))].copy_from_slice(&data_to_write[i].to_be_bytes());
                }

                7 + (registers_count as usize * 2)
            },
            #[cfg(feature="bypass")]
            RequestForm::BypassRequest(req) => {
                let len = req.to_packet(buffer).len();

                2 + len
            },
        };

        let crc_bytes = crc::gen_bytes(&buffer[..len]);
        buffer[len..(len + 2)].copy_from_slice(&crc_bytes);

        &buffer[..(len + 2)]
    }
}


#[cfg(feature="bypass")]
#[derive(Debug)]
pub struct BypassRequest<'a> {
    modbus_id: u8,
    form: &'a BypassRequestForm<'a>,
}


#[cfg(feature="bypass")]
impl<'a> BypassRequest<'a> {
    /// Creates a new Modbus RTU request instance.
    ///
    /// ---
    /// # Arguments
    /// - `modbus_id`: The Modbus slave ID
    /// - `form`: A reference to the request form
    ///
    /// ---
    /// # Returns
    /// A new instance of `Request` containing the specified Modbus ID and form
    /// 
    /// ---
    /// # Examples
    /// ```
    /// use modbus_rtu::common::{BypassRequest, BypassRequestForm};
    /// 
    /// let read_value_form = BypassRequestForm::ReadInputRegisters {
    ///     start_register: 0x0000,
    ///     registers_count: 12,
    /// };
    /// 
    /// let request = BypassRequest::new(0x01, &read_value_form);
    /// ```
    ///
    pub fn new(modbus_id: u8, form: &'a BypassRequestForm) -> BypassRequest<'a> {
        BypassRequest { modbus_id, form }
    }

    /// Writes a Modbus RTU request packet into the provided buffer and returns the corresponding slice.
    ///
    /// ---
    /// # Arguments
    /// - `buffer`: The buffer into which the packet will be written
    ///
    /// ---
    /// # Returns
    /// A slice representing the constructed Modbus RTU request packet
    ///
    /// ---
    /// # Examples
    /// ```
    /// use modbus_rtu::common::{BypassRequest, BypassRequestForm};
    ///
    /// let write_datas_form = BypassRequestForm::WriteMultipleRegisters {
    ///     start_register: 0x0001,
    ///     datas_to_write: &[0x1234, 0x5678],
    /// };
    ///
    /// let request = BypassRequest::new(0x01, &write_datas_form);
    ///
    /// let mut buffer: [u8; 256] = [0; 256];
    /// let packet = request.to_packet(&mut buffer);
    ///
    /// assert_eq!(packet, &[0x01, 0x10, 0x00, 0x01, 0x00, 0x02, 0x04, 0x12, 0x34, 0x56, 0x78, 0x49, 0x57]);
    /// ```
    /// 
    pub fn to_packet(&self, buffer: &'a mut [u8; 256]) -> &'a [u8] {
        // write modbus id
        buffer[0] = self.modbus_id;

        // write function code
        buffer[1] = self.form.get_function_code();

        // write data bytes
        let len: usize = match &self.form {
            BypassRequestForm::ReadHoldingRegisters { start_register, registers_count } |
            BypassRequestForm::ReadInputRegisters { start_register, registers_count } => {
                // write start register address
                buffer[2..4].copy_from_slice(&start_register.to_be_bytes());

                // write registers count
                buffer[4..6].copy_from_slice(&registers_count.to_be_bytes());

                // packet length without CRC bytes
                6
            },
            BypassRequestForm::WriteSingleRegister { register_address, data_to_write } => {
                // write register address
                buffer[2..4].copy_from_slice(&register_address.to_be_bytes());

                // write data to write
                buffer[4..6].copy_from_slice(&data_to_write.to_be_bytes());

                // packet length without CRC bytes
                6
            },
            BypassRequestForm::WriteMultipleRegisters { start_register, datas_to_write } => {
                // write start register address
                buffer[2..4].copy_from_slice(&start_register.to_be_bytes());

                // write registers count
                let registers_count: u16 = datas_to_write.len() as u16;
                buffer[4..6].copy_from_slice(&registers_count.to_be_bytes());

                // write bytes count
                buffer[6] = (registers_count * 2) as u8;

                // write datas to write
                for i in 0..registers_count as usize {
                    buffer[(7 + (i * 2))..=(8 + (i * 2))].copy_from_slice(&datas_to_write[i].to_be_bytes());
                }

                7 + (registers_count as usize * 2)
            },
        };

        let crc_bytes = crc::gen_bytes(&buffer[..len]);
        buffer[len..(len + 2)].copy_from_slice(&crc_bytes);

        &buffer[..(len + 2)]
    }
}
