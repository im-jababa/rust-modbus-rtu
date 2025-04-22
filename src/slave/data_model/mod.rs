mod structure;      pub use structure::DataStructure;


/// A data model for accessing values stored at unique 16-bit addresses.
#[derive(Debug)]
pub struct DataModel<const L: usize, T: Copy> {
    /// The data structure defined as a constant at compile time.
    /// The data model conforms to this structure.
    /// 
    structure: &'static DataStructure<L>,

    /// The list of values being stored.
    /// It is stored in the same form as the data structure.
    values: [T; L],
}


impl<const L: usize, T: Copy> DataModel<L, T> {
    /// Creates and initializes a new data model using the given data structure.
    ///
    /// ---
    /// # Arguments
    /// - `structure`: A reference to the constant data structure.
    /// - `initial_values`: The initial values to populate the data model.
    ///
    /// ---
    /// # Returns
    /// A new `DataModel` instance initialized with the provided structure and values.
    /// 
    /// ---
    /// # Examples
    /// ```
    /// use modbus_rtu::slave::{DataModel, DataStructure};
    /// 
    /// const STRUCTURE: DataStructure<5> = DataStructure::new([
    ///     0x0000,
    ///     0x0001,
    ///     0x0002,
    ///     0x1234,
    ///     0x5678,
    /// ]);
    /// 
    /// let data_model = DataModel::new(&STRUCTURE, [0; 5]);
    /// ```
    /// 
    pub fn new(structure: &'static DataStructure<L>, initial_values: [T; L]) -> DataModel<L, T> {
        Self { structure, values: initial_values }
    }

    /// Retrieves a value from the data model by using an address defined in the associated data structure.
    ///
    /// This method uses the address defined in the data structure to access the corresponding value
    /// stored within the data model.
    /// 
    /// ---
    /// # Arguments
    /// - `address`: The address of the value to retrieve. Only addresses defined in the data structure are allowed.
    /// 
    /// ---
    /// # Returns
    /// The value stored at the specified address.
    /// 
    /// ---
    /// # Examples
    /// ```
    /// use modbus_rtu::slave::{DataModel, DataStructure};
    /// 
    /// const STRUCTURE: DataStructure<5> = DataStructure::new([
    ///     0x0000,
    ///     0x0001,
    ///     0x0002,
    ///     0x1234,
    ///     0x5678,
    /// ]);
    /// 
    /// let data_model = DataModel::new(&STRUCTURE, [0; 5]);
    /// 
    /// assert_eq!(data_model.get_value(0x0001), 0);
    /// ```
    /// 
    /// The code below will panic at compile time.
    /// ```should_panic
    /// # use modbus_rtu::slave::{DataModel, DataStructure};
    /// # const STRUCTURE: DataStructure<5> = DataStructure::new([
    /// #     0x0000,
    /// #     0x0001,
    /// #     0x0002,
    /// #     0x1234,
    /// #     0x5678,
    /// # ]);
    /// # let data_model = DataModel::new(&STRUCTURE, [0; 5]);
    /// // Will panic!!
    /// let value = data_model.get_value(0x0003);
    /// ```
    /// 
    /// ---
    /// # Panics
    /// ...
    /// 
    pub const fn get_value(&self, address: u16) -> T {
        let index = self.structure.get(address);
        self.values[index]
    }

    /// Retrieves a value from the data model using a given address, if it exists in the structure.
    ///
    /// Unlike `get_value`, this method returns `None` instead of panicking if the address is not part of the structure.
    /// This makes it suitable for use with dynamic or external input.
    ///
    /// ---
    /// # Arguments
    /// - `address`: The address of the value to look up. The address may or may not be defined in the structure.
    ///
    /// ---
    /// # Returns
    /// An `Option<T>` containing the value if the address is found, or `None` if it is not.
    ///
    /// ---
    /// # Examples
    /// ```
    /// use modbus_rtu::slave::{DataModel, DataStructure};
    ///
    /// const STRUCTURE: DataStructure<5> = DataStructure::new([
    ///     0x0000,
    ///     0x0001,
    ///     0x0002,
    ///     0x1234,
    ///     0x5678,
    /// ]);
    ///
    /// let data_model = DataModel::new(&STRUCTURE, [10, 20, 30, 40, 50]);
    ///
    /// assert_eq!(data_model.find_value(0x0002), Some(30));
    /// assert_eq!(data_model.find_value(0x9999), None);
    /// ```
    /// 
    pub fn find_value(&self, address: u16) -> Option<T> {
        let index = self.structure.find(address)?;
        Some(self.values[index])
    }
}
