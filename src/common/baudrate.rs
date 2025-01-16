/// Represents the supported baud rates in the `Modbus RTU` protocol.
///
/// Baud rates determine the speed of communication over a serial interface. 
/// Each variant corresponds to a standard baud rate commonly used in industrial
/// and embedded systems communication.
///
/// # Features
/// - Supports conversions to `u32` and `u64`.
/// - Supports speed comparison.
/// - Calculates the 3.5-character end-of-frame duration by calling `.end_of_frame_duration()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Baudrate {
    B1200,
    B2400,
    B4800,
    B9600,
    B19200,
    B38400,
    B57600,
    B115200,
}


impl Baudrate {
    /// Minumum baudrate that supports.
    pub const MIN: Baudrate = Baudrate::B1200;

    /// Maximum baudrate that supports.
    pub const MAX: Baudrate = Baudrate::B115200;

    /// In the `Modbus RTU` protocol, the `End of Frame` is identified by a delay of at least `3.5` character times.
    /// 
    /// This function calculates the duration of the `End of Frame` based on the configured baud rate.
    /// 
    /// ***
    /// # Returns
    /// 
    /// The `End of Frame` duration as [`Duration`](https://doc.rust-lang.org/stable/core/time/struct.Duration.html),
    /// calculated using the baud rate and the protocol's 3.5-character time requirement.
    pub fn end_of_frame_duration(&self) -> core::time::Duration {
        const NANOS_PER_SEC: u64 = 1_000_000_000;
        const FRAME_SIZE: u64 = 10;
        const GAP_SIZE: f64 = 3.5;

        let nanos: u64 = ((((FRAME_SIZE as f64) * GAP_SIZE) as u64) * NANOS_PER_SEC) / u64::from(self.as_u32());
        core::time::Duration::from_nanos(nanos)
    }

    /// Maps a baudrate value to the corresponding `u32`.
    fn as_u32(&self) -> u32 {
        match self {
            Baudrate::B1200 => 1_200,
            Baudrate::B2400 => 2_400,
            Baudrate::B4800 => 4_800,
            Baudrate::B9600 => 9_600,
            Baudrate::B19200 => 19_200,
            Baudrate::B38400 => 38_400,
            Baudrate::B57600 => 57_600,
            Baudrate::B115200 => 115_200,
        }
    }

    /// Converts a numeric baudrate value to the corresponding `Baudrate` enum.
    fn from_u64(value: u64) -> Result<Self, &'static str> {
        match value {
            1_200 => Ok(Baudrate::B1200),
            2_400 => Ok(Baudrate::B2400),
            4_800 => Ok(Baudrate::B4800),
            9_600 => Ok(Baudrate::B9600),
           19_200 => Ok(Baudrate::B19200),
           38_400 => Ok(Baudrate::B38400),
           57_600 => Ok(Baudrate::B57600),
          115_200 => Ok(Baudrate::B115200),
            _ => Err("Invalid baudrate"),
        }
    }
}


impl From<&Baudrate> for u32 {
    fn from(value: &Baudrate) -> Self {
        value.as_u32()
    }
}


impl From<Baudrate> for u32 {
    fn from(value: Baudrate) -> Self {
        value.as_u32()
    }
}


impl From<&Baudrate> for u64 {
    fn from(value: &Baudrate) -> Self {
        u64::from(value.as_u32())
    }
}


impl From<Baudrate> for u64 {
    fn from(value: Baudrate) -> Self {
        u64::from(value.as_u32())
    }
}


impl TryFrom<u32> for Baudrate {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Baudrate::from_u64(value.into())
    }
}


impl TryFrom<u64> for Baudrate {
    type Error = &'static str;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Baudrate::from_u64(value)
    }
}