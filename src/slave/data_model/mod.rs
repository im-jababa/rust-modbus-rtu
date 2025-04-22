mod structure;

pub use structure::DataStructure;


/// 고유한 16비트 주소로 저장된 값에 접근할 수 있는 데이터 모델입니다.
#[derive(Debug)]
pub struct DataModel<const L: usize, T> {
    structure: &'static DataStructure<L>,
    values: [T; L],
}


impl<const L: usize, T> DataModel<L, T> {
    /// Create and initialize a new data model from data structure
    /// 
    /// ---
    /// # Args
    /// - `structure`: A constant data structure
    /// - `initial_values`: Initial value list to fill up the structures
    /// 
    /// ---
    /// # Returns
    /// A new data model from `structure` and initialzed with `initial_values`
    /// 
    pub fn new(structure: &'static DataStructure<L>, initial_values: [T; L]) -> DataModel<L, T> {
        Self { structure, values: initial_values }
    }

    
}
