/// Represents supported baudrates for Modbus communication.
///
/// Each variant maps to an internal ID used within the protocol.
/// The actual baudrate values (e.g., 9600, 115200) can be obtained via conversion.
/// 
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[repr(u16)]
pub enum Baudrate {
    BR1200 = 0,
    BR2400 = 1,
    BR4800 = 2,
    BR9600 = 3,
    BR19200 = 4,
    BR38400 = 5,
    BR57600 = 6,
    BR115200 = 7,
}


impl Baudrate {
    /// Attempts to create a `Baudrate` from its internal ID.
    ///
    /// This is used when a baudrate needs to be reconstructed from a stored or transmitted ID.
    ///
    /// ---
    /// # Args
    /// - `id`: The internal identifier corresponding to a `Baudrate`.
    ///
    /// ---
    /// # Returns
    /// `Some(Baudrate)` if the ID matches a known baudrate, otherwise `None`.
    ///
    /// ---
    /// # Examples
    /// ```
    /// use modbus_rtu::common::Baudrate;
    ///
    /// assert_eq!(Baudrate::from_id(3), Some(Baudrate::BR9600));
    /// assert_eq!(Baudrate::from_id(99), None);
    /// ```
    /// 
    pub fn from_id(id: u16) -> Option<Baudrate> {
        use Baudrate::*;
        match id {
            val if val == BR1200 as u16 => Some(BR1200),
            val if val == BR2400 as u16 => Some(BR2400),
            val if val == BR4800 as u16 => Some(BR4800),
            val if val == BR9600 as u16 => Some(BR9600),
            val if val == BR19200 as u16 => Some(BR19200),
            val if val == BR38400 as u16 => Some(BR38400),
            val if val == BR57600 as u16 => Some(BR57600),
            val if val == BR115200 as u16 => Some(BR115200),
            _ => None,
        }
    }

    /// Converts the `Baudrate` variant into its internal ID representation.
    ///
    /// This ID can be stored or transmitted and later converted back using `from_id`.
    ///
    /// ---
    /// # Returns
    /// The internal `u16` ID associated with the baudrate.
    ///
    /// ---
    /// # Examples
    /// ```
    /// use modbus_rtu::common::Baudrate;
    ///
    /// let baud = Baudrate::BR19200;
    /// assert_eq!(baud.to_id(), 4);
    /// ```
    /// 
    pub fn to_id(&self) -> u16 {
        *self as u16
    }

    /// Calculates the packet end timeout in microseconds based on the baudrate.
    ///
    /// In Modbus RTU communication, this value defines the idle time required to consider a packet as ended. (3.5 char time)
    ///
    /// ---
    /// # Returns
    /// The idle time (in microseconds) required to delimit the end of a Modbus RTU packet at this baudrate.
    ///
    /// ---
    /// # Examples
    /// ```
    /// use modbus_rtu::common::Baudrate;
    ///
    /// let baud = Baudrate::BR9600;
    /// let packet_end_us: u32 = baud.packet_end_us();
    /// ```
    /// 
    pub fn packet_end_us(&self) -> u32 {
        let bps: u32 = self.into();
        (35_000_000 + bps - 1) / bps
    }
}


impl TryFrom<u32> for Baudrate {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        use Baudrate::*;
        match value {
              1_200 => Ok(BR1200),
              2_400 => Ok(BR2400),
              4_800 => Ok(BR4800),
              9_600 => Ok(BR9600),
             19_200 => Ok(BR19200),
             38_400 => Ok(BR38400),
             57_600 => Ok(BR57600),
            115_200 => Ok(BR115200),
            _ => Err(())
        }
    }
}


impl From<Baudrate> for u32 {
    fn from(value: Baudrate) -> Self {
        match value {
            Baudrate::BR1200   =>   1_200,
            Baudrate::BR2400   =>   2_400,
            Baudrate::BR4800   =>   4_800,
            Baudrate::BR9600   =>   9_600,
            Baudrate::BR19200  =>  19_200,
            Baudrate::BR38400  =>  38_400,
            Baudrate::BR57600  =>  57_600,
            Baudrate::BR115200 => 115_200,
        }
    }
}


impl From<&Baudrate> for u32 {
    fn from(value: &Baudrate) -> Self {
        match value {
            Baudrate::BR1200   =>   1_200,
            Baudrate::BR2400   =>   2_400,
            Baudrate::BR4800   =>   4_800,
            Baudrate::BR9600   =>   9_600,
            Baudrate::BR19200  =>  19_200,
            Baudrate::BR38400  =>  38_400,
            Baudrate::BR57600  =>  57_600,
            Baudrate::BR115200 => 115_200,
        }
    }
}
