pub mod error;


use super::data::{Data, constraint::DataConstraint};


pub struct AddressArea<T: Ord + Copy, const L: usize> {
    slots: [Option<(u16, Data<T>)>; L],
}


impl<T: Ord + Copy, const L: usize> AddressArea<T, L> {
    /// Creates a new, empty `AddressArea`.
    ///
    /// ***
    /// # Returns
    ///
    /// A new `AddressArea` instance that can store up to `L` items of type `T`.
    /// 
    /// ***
    /// # Examples
    /// 
    /// ```rust
    /// use modbus_rtu::address_area::AddressArea;
    /// 
    /// let address_area: AddressArea<u16, 256> = AddressArea::new();
    /// ```
    pub fn new() -> AddressArea<T, L> {
        AddressArea { slots: [None; L] }
    }

    /// 
    pub fn add(&mut self, address: u16, initial_value: T) -> Result<(), ()> {
        // check for duplicated address
        if self.slots.iter().flatten().any(|(using_address, _)| *using_address == address) {
            return Err(());
        }
        // find empty slot
        if let Some(slot) = self.slots.iter_mut().find(|slot| slot.is_none()) {
            *slot = Some((address, Data::new(initial_value)));
            return Ok(());
        }
        // no empty slot
        Err(())
    }
}