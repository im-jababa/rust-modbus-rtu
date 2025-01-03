

pub enum AddressAreaError {
    DuplicatedAddress(u16),
    SlotsAreFull(usize),
}