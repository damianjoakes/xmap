use std::ptr;

/// Returns a `usize` corresponding to the size of a struct time the amount of space for those
/// structs to be allocated.
///
/// This is useful for when we want to get an accurate memory size to allocate for a map.
pub(in crate) fn new_capacity_of<T>(size: usize) -> usize {
    size_of::<T>() * size
}

/// Compares the memory in two pointers.
///
/// The index at which the two pointers' underlying data no longer match is returned. This will
/// return `None` if the two structures are identical.
pub(in crate) unsafe fn mem_cmp(left: *const u8, right: *const u8, size: usize) -> Option<usize> {
    // Compare each pointer byte-by-byte.
    for i in 0..size {
        if ptr::read(left.add(i)) != ptr::read(right.add(i)) {
            return Some(i);
        }
    }

    None
}