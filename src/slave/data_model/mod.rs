use crate::slave::DataStruct;


/// A data model for accessing values stored at unique 16-bit addresses.
/// 
/// ---
/// # Examples
/// ```
/// use modbus_rtu::slave::{DataModel, DataStruct};
/// 
/// // Define data structure first.
/// const DATA_STRUCTURE: DataStruct<4> = DataStruct::new([
///     0x0001,
///     0x0002,
///     0x1234,
///     0x5678,
/// ]);
/// 
/// // And create a new data model instance with initial value array.
/// let data_model = DataModel::new(&DATA_STRUCTURE, [0; 4]);
/// ```
/// 
#[derive(Debug)]
pub struct DataModel<const L: usize, T: Copy> {
    /// The data structure defined as a constant at compile time.
    /// The data model conforms to this structure.
    /// 
    structure: &'static DataStruct<L>,

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
    /// use modbus_rtu::slave::{DataModel, DataStruct};
    /// 
    /// const STRUCTURE: DataStruct<5> = DataStruct::new([
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
    pub fn new(structure: &'static DataStruct<L>, initial_values: [T; L]) -> DataModel<L, T> {
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
    /// use modbus_rtu::slave::{DataModel, DataStruct};
    /// 
    /// const STRUCTURE: DataStruct<5> = DataStruct::new([
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
    /// # use modbus_rtu::slave::{DataModel, DataStruct};
    /// # const STRUCTURE: DataStruct<5> = DataStruct::new([
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
    /// use modbus_rtu::slave::{DataModel, DataStruct};
    ///
    /// const STRUCTURE: DataStruct<5> = DataStruct::new([
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

    /// Sets a value in the data model at the specified index.
    ///
    /// This function assumes the index is already validated and corresponds to a valid entry in the model.
    ///
    /// ---
    /// # Args
    /// - `index`: The index at which to set the value. This must be valid, as obtained from `get_index`.
    /// - `value`: The value to be set at the specified index.
    ///
    /// ---
    /// # Examples
    /// ```
    /// # use modbus_rtu::slave::{DataModel, DataStruct};
    /// # 
    /// # const DATA_STRUCTURE: DataStruct<4> = DataStruct::new([
    /// #     0x0000,
    /// #     0x0001,
    /// #     0x1234,
    /// #     0x5678,
    /// # ]);
    /// # 
    /// # let mut data_model = DataModel::new(&DATA_STRUCTURE, [0; 4]);
    /// # 
    /// // This is find.
    /// let i = data_model.get_index(0x0000);
    /// data_model.set_value(i, 5);
    /// 
    /// // This is fine too.
    /// if let Some(i) = data_model.find_index(0x0008) {
    ///     data_model.set_value(i, 5);
    /// }
    /// 
    /// // But this might panic.
    /// data_model.set_value(3, 5);
    /// ```
    /// 
    /// ---
    /// # Panics
    /// Panics if the index is out of bounds.
    ///
    pub fn set_value(&mut self, index: usize, value: T) {
        self.values[index] = value;
    }

    /// Retrieves the internal index for a given address defined in the data structure.
    ///
    /// This is useful when you want to perform index-based operations on the values array.
    ///
    /// ---
    /// # Args
    /// - `address`: The address whose index should be retrieved. Must be present in the structure.
    ///
    /// ---
    /// # Returns
    /// The index corresponding to the given address.
    ///
    /// ---
    /// # Panics
    /// Panics at compile time if the address is not found in the structure.
    ///
    pub const fn get_index(&self, address: u16) -> usize {
        self.structure.get(address)
    }

    /// Attempts to find the index for a given address, if it exists in the data structure.
    ///
    /// This method is safe to use with dynamic or externally provided addresses.
    ///
    /// ---
    /// # Args
    /// - `address`: The address to search for.
    ///
    /// ---
    /// # Returns
    /// `Some(index)` if the address is present, or `None` otherwise.
    ///
    pub fn find_index(&self, address: u16) -> Option<usize> {
        self.structure.find(address)
    }

    /// Checks whether the data model is empty.
    ///
    /// This returns `true` if the data model contains no entries, which occurs when its length `L` is zero.
    ///
    /// ---
    /// # Returns
    /// `true` if the data model is empty, `false` otherwise.
    ///
    /// ---
    /// # Examples
    /// ```
    /// use modbus_rtu::slave::DataModel;
    ///
    /// let empty_model = DataModel::<0, u16>::empty();
    /// assert!(empty_model.is_empty());
    /// ```
    ///
    pub fn is_empty(&self) -> bool {
        L == 0
    }
}


impl<T: Copy> DataModel<0, T> {
    /// Creates and returns an empty data model with no stored values.
    ///
    /// This is useful when a data model is required but no data is needed or used.
    ///
    /// ---
    /// # Returns
    /// An empty `DataModel` instance with no associated data.
    ///
    /// ---
    /// # Examples
    /// ```
    /// use modbus_rtu::slave::{ModbusSlave, DataModel};
    ///
    /// let holding_registers = DataModel::empty();
    /// let input_registers = DataModel::empty();
    /// 
    /// // Create modbus slave instance with zero registers
    /// let modbus_slave = ModbusSlave::new(0x01, holding_registers, input_registers);
    /// ```
    ///
    pub fn empty() -> DataModel<0, T> {
        DataModel { structure: &DataStruct::EMPTY, values: [] }
    }
}
