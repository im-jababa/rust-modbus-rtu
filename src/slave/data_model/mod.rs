pub mod structure;

use structure::DataStructure;


#[derive(Debug)]
pub struct DataModel<const L: usize, T> {
    structure: &'static DataStructure<L>,
    values: [T; L],
}


impl<const L: usize, T> DataModel<L, T> {
    pub fn new(structure: &'static DataStructure<L>, initial_values: [T; L]) -> DataModel<L, T> {
        Self { structure, values: initial_values }
    }

    
}