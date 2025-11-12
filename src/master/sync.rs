//! Blocking Modbus RTU master backed by the `serialport` crate.

use crate::{Request, Response};


/// Blocking Modbus RTU master that enforces Modbus idle timing rules between frames.
#[derive(Debug)]
pub struct Master {
    /// Serial port handle used for request/response traffic.
    port: Box<dyn serialport::SerialPort>,

    /// Timestamp of the last transmitted frame, used to honor the 3.5-char gap.
    last_tx: std::time::Instant,

    /// Cached baud rate so higher-level code can inspect the active speed.
    baud_rate: u32,
}


impl Master {
    /// Builds a master configured for an RS-485 style setup (8N1, blocking I/O).
    ///
    /// The port timeout is pinned to the Modbus RTU silent interval (T3.5) for
    /// the supplied baud rate so that the reader can detect frame boundaries.
    ///
    /// ---
    /// # Examples
    /// ```ignore
    /// use modbus_rtu::Master;
    ///
    /// # fn demo() -> serialport::Result<()> {
    /// let master = Master::new_rs485("/dev/ttyUSB0", 9_600)?;
    /// assert_eq!(master.baud_rate(), 9_600);
    /// # Ok(())
    /// # }
    /// ```
    /// 
    pub fn new_rs485(path: &str, baud_rate: u32) -> serialport::Result<Self> {
        let port = serialport::new(path, baud_rate)
            .data_bits(serialport::DataBits::Eight)
            .parity(serialport::Parity::None)
            .stop_bits(serialport::StopBits::One)
            .timeout(Self::idle_time_rs485(baud_rate))
            .open()?;
        Ok(Self { port, last_tx: (std::time::Instant::now() - Self::idle_time_rs485(baud_rate)), baud_rate })
    }

    /// Returns the baud rate currently configured on the serial link.
    ///
    /// ---
    /// # Examples
    /// ```ignore
    /// use modbus_rtu::Master;
    ///
    /// # fn demo() -> serialport::Result<()> {
    /// let master = Master::new_rs485("/dev/ttyUSB0", 38_400)?;
    /// assert_eq!(master.baud_rate(), 38_400);
    /// # Ok(())
    /// # }
    /// ```
    /// 
    pub fn baud_rate(&self) -> u32 {
        self.baud_rate
    }

    /// Updates the serial baud rate and matching Modbus idle timeout.
    ///
    /// ---
    /// # Examples
    /// ```ignore
    /// use modbus_rtu::Master;
    ///
    /// # fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut master = Master::new_rs485("/dev/ttyUSB0", 9_600)?;
    /// master.set_baudrate(19_200)?;
    /// assert_eq!(master.baud_rate(), 19_200);
    /// # Ok(())
    /// # }
    /// ```
    /// 
    pub fn set_baudrate(&mut self, baud_rate: u32) -> serialport::Result<()> {
        self.port.set_baud_rate(baud_rate)?;
        self.port.set_timeout(Self::idle_time_rs485(baud_rate))?;
        self.baud_rate = baud_rate;
        self.last_tx = std::time::Instant::now();
        Ok(())
    }

    /// Sends a Modbus RTU request and waits for the corresponding response.
    ///
    /// Broadcast requests return immediately after the frame is flushed because
    /// the Modbus RTU spec forbids responses to slave id 0.
    ///
    /// ---
    /// # Examples
    /// ```ignore
    /// use modbus_rtu::{Function, Master, Request};
    ///
    /// # fn demo() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut master = Master::new_rs485("/dev/ttyUSB0", 19_200)?;
    /// let func = Function::ReadHoldingRegisters { starting_address: 0x0000, quantity: 2 };
    /// let request = Request::new(0x01, &func, std::time::Duration::from_millis(200));
    /// let response = master.send(&request)?;
    /// assert!(response.is_success());
    /// # Ok(())
    /// # }
    /// ```
    /// 
    pub fn send(&mut self, req: &Request) -> crate::Result {
        while self.last_tx.elapsed() <= Self::idle_time_rs485(self.baud_rate) {
            std::thread::sleep(core::time::Duration::from_micros(1));
        }
        let frame = req.to_bytes().map_err(|e| crate::error::Error::Request(e))?;
        self.port.clear(serialport::ClearBuffer::Output).map_err(|e| crate::error::Error::IO(e.into()))?;
        self.write(&frame)?;
        if req.is_broadcasting() {
            return Ok(Response::Success);
        }
        std::thread::sleep(Self::idle_time_rs485(self.baud_rate));
        let mut buf: [u8; 256] = [0; 256];
        let len = self.read(&mut buf, req.timeout())?;
        if len == 0 {
            return Err(crate::error::Error::IO(std::io::ErrorKind::TimedOut.into()));
        }
        Response::from_bytes(req, &buf[0..len]).map_err(|e| crate::error::Error::Response(e))
    }

    /// Writes a Modbus frame to the serial port and records the transmit instant.
    fn write(&mut self, frame: &[u8]) -> Result<(), crate::error::Error> {
        self.port.write_all(frame)
            .map_err(|e| crate::error::Error::IO(e.into()))?;
        self.last_tx = std::time::Instant::now();
        Ok(())
    }

    /// Reads bytes until the slave stops responding or `buf` fills up.
    fn read(&mut self, buf: &mut [u8], timeout: core::time::Duration) -> Result<usize, crate::error::Error> {
        let start = std::time::Instant::now();
        let mut len: usize = 0;
        while start.elapsed() <= timeout {
            let n = match self.port.read(&mut buf[len..]) {
                Ok(n) => n,
                Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => if len == 0 { continue } else { break },
                Err(e) => return Err(crate::error::Error::IO(e.into())),
            };
            len += n;
            if len >= buf.len() {
                break;
            }
        }
        Ok(len)
    }

    /// Computes the Modbus RTU T3.5 idle time for a link running 8N1 encoding.
    fn idle_time_rs485(baud_rate: u32) -> core::time::Duration {
        const BITS_PER_CHAR: f64 = 10.0;
        let seconds = 3.5 * BITS_PER_CHAR / baud_rate as f64;
        core::time::Duration::from_secs_f64(seconds)
    }
}
