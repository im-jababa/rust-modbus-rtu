/// Represents a statically defined set of Modbus RTU data addresses.
///
/// The addresses must be:
/// - Known at compile time
/// - Strictly increasing (ordered)
/// - Unique
///
/// This structure is intended to be defined as a global constant/static value,
/// and should not be instantiated dynamically.
/// 
#[derive(Debug)]
pub struct DataStructure<const L: usize>([u16; L]);

impl<const L: usize> DataStructure<L> {
    /// Creates a new `DataStructure` after validating the input addresses.
    ///
    /// This function checks that the addresses are strictly ordered and unique at compile time.
    /// It will panic if the addresses are not ordered or contain duplicates.
    ///
    /// ---
    /// # Arguments
    /// - `addresses`: The list of Modbus RTU register addresses to be stored in the structure.
    ///
    /// ---
    /// # Returns
    /// A new `DataStructure` instance if the validation passes.
    /// 
    pub const fn new(addresses: [u16; L]) -> DataStructure<L> {
        let _ = Self::validate(&addresses);
        Self(addresses)
    }

    /// Get the index of the specified address using a manual binary search approach.
    ///
    /// This function performs a binary search on the statically defined address list.
    /// It will panic if the address is not found in the list.
    ///
    /// ---
    /// # Arguments
    /// - `address`: The address to look for in the data structure.
    ///
    /// ---
    /// # Returns
    /// The index of the address in the list if it exists, or panics if not found.
    ///
    /// ---
    /// # Panics
    /// This function will panic if the specified address is not found in the address list.
    /// The panic message will indicate that the address is not registered and should be checked for validity.
    /// 
    pub const fn get(&self, address: u16) -> usize {
        let mut left = 0;
        let mut right = self.0.len() - 1;
        
        while left <= right {
            let mid = (left + right) / 2;
            if self.0[mid] == address {
                return mid;
            } else if self.0[mid] < address {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        
        panic!("Address not found in the address list. Ensure the address is valid and registered.");
    }

    /// Finds the index of the specified address using the binary search algorithm.
    ///
    /// This function performs a binary search on the statically defined address list. 
    /// It returns the index of the address if found.
    ///
    /// ---
    /// # Arguments
    /// - `address`: The address to search for in the data structure.
    ///
    /// ---
    /// # Returns
    /// `Some(index)` if the address exists in the list, or `None` if not found.
    ///
    /// ---
    /// # Panics
    /// This function does not panic. It returns `None` if the address is not found.
    /// 
    pub fn find(&self, address: u16) -> Option<usize> {
        self.0.binary_search(&address).ok()
    }

    /// Retrieves the address at the specified index.
    ///
    /// This function accesses the address stored at the given index in the data structure's array.
    /// It is a simple getter function that does not perform any validation.
    ///
    /// ---
    /// # Arguments
    /// - `index`: The index of the address to retrieve. The number must be derived from the logical relationships defined within the data structure.
    ///
    /// ---
    /// # Returns
    /// The `u16` address located at the given index.
    ///
    /// ---
    /// # Panics
    /// This function will panic if the `index` is out of bounds for the address list.
    /// The number must be derived from the logical relationships defined within the data structure.
    ///
    pub fn get_address_by_index(&self, index: usize) -> u16 {
        self.0[index]
    }

    /// Validates that the address list is strictly increasing and unique.
    ///
    /// ---
    /// # Arguments
    /// - `addresses`: The list of addresses to validate.
    ///
    /// ---
    /// # Returns
    /// `true` if the addresses are valid (ordered and unique), otherwise panics.
    ///
    /// ---
    /// # Panics
    /// This function will panic if the addresses are not strictly increasing or contain duplicates.
    const fn validate(addresses: &[u16; L]) -> bool {
        if addresses.len() < 2 {
            return true;
        }

        let mut i = 0;
        while i < addresses.len() - 1 {
            if addresses[i] >= addresses[i + 1] {
                panic!("Addresses for data structure must be ordered and unique. Found invalid order at index.");
            }
            i += 1;
        }
        true
    }
}