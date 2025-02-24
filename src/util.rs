/// Returns a `usize` corresponding to the size of a struct time the amount of space for those
/// structs to be allocated.
///
/// This is useful for when we want to get an accurate memory size to allocate for a map.
pub(in crate) fn new_capacity_of<T>(size: usize) -> usize {
    size_of::<T>() * size
}