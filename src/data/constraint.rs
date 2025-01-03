/// # DataConstraint
/// 
/// Modbus RTU data constraint.
/// 
/// When the Master device requests to write new data,
/// the data undergoes constraint validation.
/// If an attempt is made to write data that violates these constraints,
/// the Slave device returns error code 2 (Illegal Data) to the Master device.
#[derive(Debug, Clone, Copy)]
pub enum DataConstraint<T: Ord + Clone + Copy> {
    
    /// This constraint allows only a single specific value.
    /// 
    /// ***
    /// # Value
    /// 
    /// The only specific value that is permitted.
    /// 
    /// ***
    /// # Examples
    /// 
    /// ```rust
    /// use modbus_rtu::data::constraint::DataConstraint;
    /// 
    /// let constraint: DataConstraint<i32> = DataConstraint::Only(10);
    /// 
    /// assert_eq!(constraint.validate(&10), true);
    /// assert_eq!(constraint.validate(&5), false);
    /// ```
    Only(T),

    /// This constraint allows values within a specific range.
    /// 
    /// ***
    /// # Values
    /// 
    /// - `min`: Smallest value that this constraint allows.
    /// - `max`: Largest value that this constraint allows.
    /// 
    /// ***
    /// # Examples
    /// 
    /// ```rust
    /// use modbus_rtu::data::constraint::DataConstraint;
    /// 
    /// let constraint: DataConstraint<i32> = DataConstraint::Range { min: -10, max: 10 };
    /// 
    /// assert_eq!(constraint.validate(&0), true);
    /// assert_eq!(constraint.validate(&-15), false);
    /// assert_eq!(constraint.validate(&20), false);
    /// ```
    Range { min: T, max: T },

    /// This constraint uses a custom function to validate values.
    /// 
    /// ***
    /// # Value
    /// 
    /// The function that checks value validation.
    /// 
    /// ***
    /// # Examples
    /// 
    /// ```rust
    /// use modbus_rtu::data::constraint::DataConstraint;
    /// 
    /// // Define a custom validation function
    /// fn is_even(val: &i32) -> bool {
    ///     val % 2 == 0
    /// }
    /// 
    /// let constraint: DataConstraint<i32> = DataConstraint::Custom(is_even);
    /// 
    /// assert_eq!(constraint.validate(&8), true);
    /// assert_eq!(constraint.validate(&7), false);
    /// ```
    Custom(fn(&T) -> bool),
}

impl<T: Ord + Clone + Copy> DataConstraint<T> {
    /// Checks if a given value satisfies the constraint.
    /// 
    /// ***
    /// # Args
    /// 
    /// - `value`: The value to validate.
    /// 
    /// ***
    /// # Returns
    /// 
    /// - `true` if the value satisfies the constraint.
    /// - `false` otherwise.
    /// 
    /// ***
    /// # Examples
    /// 
    /// ```rust
    /// use modbus_rtu::data::constraint::DataConstraint;
    /// 
    /// let constraint: DataConstraint<i32> = DataConstraint::Only(10);
    /// 
    /// assert_eq!(constraint.validate(&10), true);
    /// assert_eq!(constraint.validate(&5), false);
    /// ```
    pub fn validate(&self, value: &T) -> bool {
        match self {
            DataConstraint::Only(expected) => value == expected,
            DataConstraint::Range { min, max } => min <= value && value <= max,
            DataConstraint::Custom(func) => func(value),
        }
    }
}