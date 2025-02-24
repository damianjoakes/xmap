use std::alloc::Layout;
use std::ptr;
use crate::error::MapError;

#[derive(Debug)]
pub struct KeyValue<'a, K, V> {
    key: &'a K,
    value: &'a V,
}

impl<'a, K, V> KeyValue<'a, K, V> {
    pub fn key(&self) -> &'a K {
        &self.key
    }

    pub fn value(&self) -> &'a V {
        &self.value
    }
}

#[derive(Debug)]
pub struct CIndexMap<K, V> {
    key_layout: Layout,
    val_layout: Layout,
    keys: *mut K,
    values: *mut V,

    /// The total allocated memory for this map.
    ///
    /// Everything in the range `self.pos + 1..self.cap` is not uninitialized, but may be old
    /// or unwanted.
    cap: isize,

    /// Position of the cursor where the **last valid value** was inserted. `keys[pos]` and
    /// `values[pos]` will always be initialized with valid data.
    ///
    /// All elements 0 to `self.pos` are guaranteed to be initialized.
    pos: isize,
}

impl<K, V> CIndexMap<K, V> {
    /// Constructs a new `CIndexMap`.
    ///
    /// # Panics
    /// When `size_of::<K>` or `size_of::<T>` is 0.
    pub fn new() -> CIndexMap<K, V> {
        if size_of::<K>() == 0 || size_of::<V>() == 0 {
            panic!("Cannot initialize CIndexMap with ZSTs!");
        }

        let key_layout = unsafe {
            Layout::from_size_align(size_of::<K>() * 1, align_of::<K>()).unwrap()
        };
        let val_layout = unsafe {
            Layout::from_size_align(size_of::<V>() * 1, align_of::<V>()).unwrap()
        };

        // SAFETY:
        // ZSTs are not supported, this point in the initializing cannot be reached
        // if a ZST is provided.
        let key_ptr = unsafe {
            std::alloc::alloc(key_layout)
        } as *mut K;

        // SAFETY:
        // ZSTs are not supported, this point in the initializing cannot be reached
        // if a ZST is provided.
        let val_ptr = unsafe {
            std::alloc::alloc(val_layout)
        } as *mut V;

        CIndexMap {
            key_layout,
            val_layout,
            keys: key_ptr,
            values: val_ptr,
            cap: 1,
            pos: -1,
        }
    }

    /// Inserts a new key/value pair into the map.
    ///
    /// This function returns `Ok(())` if the key/value pair was inserted successfully.
    pub fn insert(&mut self, key: K, value: V) -> crate::result::Result<()> {
        if (self.pos + 1) >= self.cap {
            let new_cap = self.cap + 2;

            let new_key_ptr = unsafe {
                std::alloc::realloc(
                    self.keys as *mut u8,
                    self.key_layout,
                    size_of::<K>() * (new_cap as usize)
                ) as *mut K
            };

            let new_val_ptr = unsafe {
                std::alloc::realloc(
                    self.values as *mut u8,
                    self.val_layout,
                    size_of::<V>() * (new_cap as usize)
                ) as *mut V
            };

            self.keys = new_key_ptr;
            self.values = new_val_ptr;
            self.cap = new_cap;
        }

        self.pos += 1;

        // SAFETY:
        // The data was properly aligned during initialization.
        // The memory being pointed to was verified to be between 0 (the pointer starting position)
        // and `self.cap`, which is the current amount of allocated space.
        unsafe {
            self.keys.offset(self.pos).write(key);
            self.values.offset(self.pos).write(value);
        }

        Ok(())
    }

    /// Removes an element at the specified index.
    pub fn remove(&mut self, index: usize) -> crate::result::Result<()> {
        if index > (self.pos as usize) {
            Err(MapError {})
        } else {
            // SAFETY:
            // We've determined that the index requested fits within the range of valid data.
            unsafe {
                // Shift all elements between `index` and `self.pos` back by one by copying
                // the pointer and overwriting it at the position specified by `index`.
                ptr::copy(
                    self.keys.offset((index as isize) + 1),
                    self.keys.offset(index as isize),
                    (self.pos as usize - index)
                );

                ptr::copy(
                    self.values.offset((index as isize) + 1),
                    self.values.offset(index as isize),
                    self.pos as usize - index
                );
            }

            // Decrement self.pos to ensure this is always set as the position of the
            // last valid inserted value.
            self.pos -= 1;
            Ok(())
        }
    }

    /// Returns the key/object pair based on insertion order.
    pub fn index(&self, index: usize) -> Option<KeyValue<K, V>> {
        if index > (self.pos as usize) {
            None
        } else {
            // SAFETY:
            // All elements `0..self.pos` should be already be initialized.
            unsafe {
                Some(
                    KeyValue {
                        key: &*self.keys.offset(index as isize),
                        value: &*self.values.offset(index as isize)
                    }
                )
            }
        }
    }
}