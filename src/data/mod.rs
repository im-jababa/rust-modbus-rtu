pub mod constraint;

use constraint::DataConstraint;


/// Represents Modbus RTU data with an optional constraint.
/// 
/// This struct holds a value of generic type `T` and an optional constraint
/// that can be applied to validate the value.
#[cfg(feature = "slave")]
#[derive(Debug, Clone, Copy)]
pub struct Data<T: Ord + Copy> {

    /// The value that this Data struct holds.
    value: T,

    /// An optional constraint applied to the value.
    /// 
    /// If set, the value must satisfy the constraint.
    /// Default is `None`, meaning no constraints are applied.
    constraint: Option<DataConstraint<T>>,
}


impl<T: Ord + Copy> Data<T> {
    /// Creates a new instance of `Data` with the given initial value and no constraints.
    /// 
    /// ***
    /// # Args
    /// 
    /// - `initial_value`: The initial value to store in the `Data` struct.
    /// 
    /// ***
    /// # Returns
    /// 
    /// A new `Data` instance containing the `initial_value` and no constraints.
    /// 
    /// ***
    /// # Examples
    /// 
    /// ```rust
    /// use modbus_rtu::data::Data;
    /// 
    /// let data: Data<i32> = Data::new(0);
    /// ```
    pub fn new(initial_value: T) -> Data<T> {
        Data { value: initial_value, constraint: None }
    }

    /// Sets a constraint for the `Data` struct after validating the current value against any existing constraint.
    /// 
    /// ***
    /// # Args
    /// 
    /// - `constraint`: The `DataConstraint` to apply to the value.
    /// 
    /// ***
    /// # Returns
    /// 
    /// - `Ok(Data<T>)`: The updated `Data` instance with the new constraint if validation succeeds.
    /// - `Err(())`: Returns an error if the current value does not satisfy the existing constraint.
    /// 
    /// ***
    /// # Examples
    /// 
    /// ```rust
    /// use modbus_rtu::data::{Data, constraint::DataConstraint};
    /// 
    /// let constrainted_data: Data<i32> = Data::new(10)
    ///     .with_constraint(DataConstraint::Only(10))
    ///     .unwrap();
    /// ```
    pub fn with_constraint(mut self, constraint: DataConstraint<T>) -> Result<Data<T>, ()> {
        if let Some(constraint) = self.constraint {
            if constraint.validate(&self.value) == false {
                return Err(());
            }
        }
        self.constraint = Some(constraint);
        Ok(self)
    }

    /// Retrieves the current value stored in the `Data` struct.
    /// 
    /// ***
    /// # Returns
    /// 
    /// The value stored within the struct.
    /// 
    /// ***
    /// # Examples
    /// 
    /// ```rust
    /// use modbus_rtu::data::Data;
    /// 
    /// let data: Data<i32> = Data::new(7);
    /// 
    /// assert_eq!(data.get_value(), 7);
    /// ```
    pub fn get_value(&self) -> T {
        self.value
    }

    /// Sets a new value for the `Data` struct after validating it against the existing constraint.
    ///
    /// ***
    /// ## Args
    ///
    /// - `value`: A reference to the new value to set.
    ///
    /// ***
    /// # Returns
    ///
    /// - `Ok(())`: If the new value satisfies the existing constraint or if no constraint is set.
    /// - `Err(())`: If the new value does not satisfy the existing constraint.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use modbus_rtu::data::{Data, constraint::DataConstraint};
    ///
    /// let mut data: Data<i32> = Data::new(10)
    ///     .with_constraint(DataConstraint::Only(10))
    ///     .unwrap();
    /// 
    /// assert!(data.set_value(&10).is_ok());
    /// assert!(data.set_value(&5).is_err());
    /// ```
    pub fn set_value(&mut self, value: &T) -> Result<(), ()> {
        if let Some(constraint) = self.constraint {
            if constraint.validate(value) == false {
                return Err(());
            }
        }
        self.value = *value;
        Ok(())
    }
}