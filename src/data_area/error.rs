/// Error types that can occur when adding a new data to the data area.
#[cfg(feature = "slave")]
pub enum DataAreaError {
    /// The same data address already exists in the data area. (duplicated data address)
    DuplicatedAddress(u16),

    /// The data area is full. (max capacity)
    SlotsAreFull(usize),
}