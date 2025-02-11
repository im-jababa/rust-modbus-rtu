pub struct Data<T>
where
    T: Copy + Eq + Ord,
{
    value: T,
    constraint: fn(&T) -> bool,
}


impl<T> Data<T>
where
    T: Copy + Eq + Ord
{
    pub fn new(value: T) -> Data<T> {
        Data { value, constraint: Data::<T>::constraint_always }
    }

    fn constraint_always(_: &T) -> bool {
        true
    }
}