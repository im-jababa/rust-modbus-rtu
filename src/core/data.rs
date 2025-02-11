/// Modbus data struct can hold type `T`
pub struct Data<T>
where
    T: Copy + Eq + Ord,
{
    /// Value that holding.
    value: T,

    /// Value constraint when the master try to write this data.
    /// value is only constrained when the master try to write.
    /// Constraint is not checked when the value changed on your firmware.
    constraint: fn(&T) -> bool,
}