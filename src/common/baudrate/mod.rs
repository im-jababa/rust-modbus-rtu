#[cfg(test)]
mod test;


/// Represents supported baudrates for Modbus communication.
///
/// Each variant maps to an internal ID used within the protocol.
/// The actual baudrate values (e.g., 9600, 115200) can be obtained via conversion.
/// 
/// ---
/// # Supports
/// - [`u32`] -> [`Baudrate`]
/// - [`u64`] -> [`Baudrate`]
/// - [`Baudrate`] -> [`u32`]
/// - [`Baudrate`] -> [`u64`]
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
    /// Attempts to create a [`Baudrate`] from its internal ID.
    ///
    /// This is used when a baudrate needs to be reconstructed from a stored or transmitted ID.
    ///
    /// ---
    /// # Args
    /// - `id`: The internal identifier corresponding to a [`Baudrate`].
    ///
    /// ---
    /// # Returns
    /// [`Some`] if the ID matches a known baudrate, otherwise [`None`].
    ///
    /// ---
    /// # Examples
    /// ```
    /// use modbus_rtu::Baudrate;
    ///
    /// let baudrate: Baudrate = Baudrate::from_id(3).unwrap();
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

    /// Converts the [`Baudrate`] variant into its internal ID representation.
    ///
    /// This ID can be stored or transmitted and later converted back using [`Baudrate::from_id`].
    ///
    /// ---
    /// # Returns
    /// The internal [`u16`] ID associated with the baudrate.
    ///
    /// ---
    /// # Examples
    /// ```
    /// use modbus_rtu::Baudrate;
    ///
    /// let baudrate_id: u16 = Baudrate::BR9600.to_id();
    /// ```
    /// 
    pub const fn to_id(&self) -> u16 {
        *self as u16
    }

    /// Calculates the packet end timeout in microseconds based on the baudrate.
    ///
    /// In Modbus RTU communication, this value defines the idle time required to consider a packet as ended. (3.5 char time in 8N1)
    ///
    /// ---
    /// # Returns
    /// The idle time (in microseconds) required to delimit the end of a Modbus RTU packet at this baudrate.
    ///
    /// ---
    /// # Examples
    /// ```
    /// use modbus_rtu::Baudrate;
    ///
    /// let baudrate = Baudrate::BR9600;
    /// let packet_end_us: u64 = baudrate.packet_end_us();
    /// ```
    /// 
    pub const fn packet_end_us(&self) -> u64 {
        let bps: u64 = self.to_u32() as u64;
        (35_000_000 + bps - 1) / bps
    }

    /// [`Baudrate`] : [`u32`] map
    /// 
    /// ---
    /// # Returns
    /// [`u32`] value corresponding to [`Baudrate`]
    /// 
    const fn to_u32(&self) -> u32 {
        match self {
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


// u32 -> Baudrate
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


// u64 -> Baudrate
impl TryFrom<u64> for Baudrate {
    type Error = ();

    fn try_from(value: u64) -> Result<Self, Self::Error> {
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


// Baudrate -> u32
impl From<Baudrate> for u32 {
    fn from(value: Baudrate) -> Self {
        value.to_u32()
    }
}


// &Baudrate -> u32
impl From<&Baudrate> for u32 {
    fn from(value: &Baudrate) -> Self {
        value.to_u32()
    }
}


// Baudrate -> u64
impl From<Baudrate> for u64 {
    fn from(value: Baudrate) -> Self {
        value.to_u32() as u64
    }
}


// &Baudrate -> u64
impl From<&Baudrate> for u64 {
    fn from(value: &Baudrate) -> Self {
        value.to_u32() as u64
    }
}


// Display
#[cfg(not(feature = "no_std"))]
impl std::fmt::Display for Baudrate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let num: u32 = self.into();
        write!(f, "Baudrate({})", num)
    }
}
