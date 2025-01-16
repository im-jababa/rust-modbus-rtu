pub mod error;

use super::data::Data;
use error::DataAreaError;


/// A data area capable of storing `L` number of `T` type.
/// 
/// Each value is distinguished by a unique 16-bit Data.
/// 
/// ***
/// # Examples
/// 
/// ```rust
/// ```
#[cfg(feature = "slave")]
pub struct DataArea<T: Ord + Copy, const L: usize> {
    slots: [Option<(u16, Data<T>)>; L],
}


impl<T: Ord + Copy, const L: usize> DataArea<T, L> {
    /// Creates a new, empty `DataArea`.
    ///
    /// ***
    /// # Returns
    ///
    /// A new `DataArea` instance that can store up to `L` items of type `T`.
    /// 
    /// ***
    /// # Examples
    /// 
    /// ```rust
    /// use modbus_rtu::data_area::DataArea;
    /// 
    /// let Data_area: DataArea<u16, 256> = DataArea::new();
    /// ```
    pub fn new() -> DataArea<T, L> {
        DataArea { slots: [None; L] }
    }

    /// 
    pub fn put(&mut self, address: u16, data: Data<T>) -> Result<(), DataAreaError> {
        // check for duplicated Data
        if self.slots.iter().flatten().any(|(using_address, _)| *using_address == address) {
            return Err(DataAreaError::DuplicatedAddress(address));
        }

        // find empty slot
        if let Some(slot) = self.slots.iter_mut().find(|slot| slot.is_none()) {
            *slot = Some((address, data));
            return Ok(());
        }

        // no empty slot
        Err(DataAreaError::SlotsAreFull(self.slots.len()))
    }
}