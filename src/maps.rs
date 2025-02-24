//! The primary maps module for `x-map`. This module contains all code relating to the various
//! maps that are packaged as part of `x-map`.
//!
//! Many map types in this module utilize raw pointers. This unsafe code is managed internally by each
//! map contained within, and this unsafe code should be abstracted away from library users.
//!
//! To support multiple environments, many maps will also contain functions for low-level operations,
//! so that in environments such as `#![no_std]` environments, these maps are still usable.
//!
//! As a rule of thumb, all functions in which it is expected to retrieve a value will retrieve a
//! reference to that value, not a copy of it.

use crate::error::{CIndexMapError, MapErrorKind};
use crate::util::mem_cmp;
use core::alloc::Layout;
use core::fmt::Debug;
use core::ptr;

#[derive(Debug)]
pub struct CIndexMap<K, V> {
    /// The initial layout used to initialize this map's keys store.
    key_layout: Layout,

    /// The initial layout used to initialize this map's values store.
    val_layout: Layout,

    /// A pointer to a space in memory in which keys will be stored at.
    keys: *mut K,

    /// A pointer to a space in memory in which values will be stored at.
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
                    size_of::<K>() * (new_cap as usize),
                ) as *mut K
            };

            let new_val_ptr = unsafe {
                std::alloc::realloc(
                    self.values as *mut u8,
                    self.val_layout,
                    size_of::<V>() * (new_cap as usize),
                ) as *mut V
            };

            if (new_val_ptr == ptr::null_mut()) || (new_key_ptr == ptr::null_mut()) {
                return Err(CIndexMapError::new(
                    MapErrorKind::AllocationError,
                    "Error when attempting to allocate map memory."
                ));
            } else {
                self.keys = new_key_ptr;
                self.values = new_val_ptr;
                self.cap = new_cap;
            }
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
            Err(
                CIndexMapError::new(
                    MapErrorKind::AccessError,
                    "Attempted to access a map index that surpasses the bounds of the current map."
                )
            )
        } else {
            // SAFETY:
            // We've determined that the index requested fits within the range of valid data.
            unsafe {
                // Shift all elements between `index` and `self.pos` back by one by copying
                // the pointer and overwriting it at the position specified by `index`.
                ptr::copy(
                    self.keys.offset((index as isize) + 1),
                    self.keys.offset(index as isize),
                    (self.pos as usize - index),
                );

                ptr::copy(
                    self.values.offset((index as isize) + 1),
                    self.values.offset(index as isize),
                    self.pos as usize - index,
                );
            }

            // Decrement self.pos to ensure this is always set as the position of the
            // last valid inserted value.
            self.pos -= 1;
            Ok(())
        }
    }

    /// Returns the key to the entry at the specified index.
    ///
    /// Use `CIndexMap::get` to get the value of the entry based off of the key.
    pub fn index(&self, index: usize) -> crate::result::Result<&K> {
        if index > (self.pos as usize) {
            Err(
                CIndexMapError::new(
                    MapErrorKind::AccessError,
                    "Attempted to access a map index that surpasses the bounds of the current map."
                )
            )
        } else {
            // SAFETY:
            // All elements `0..self.pos` should be already be initialized.
            unsafe {
                Ok(
                    &*self.keys.offset(index as isize)
                )
            }
        }
    }

    /// Gets the value associated with the specified key.
    ///
    /// This function is preferred for when the underlying Key/Value types for this map cannot,
    /// for some reason, implement `PartialEq`.
    ///
    /// This performs a raw memory comparison between the supplied value and the keys in the
    /// current map. This comparison doesn't work on a lot of dynamically sized types, like `String`
    /// and `Vec`.
    /// # Correct usage
    /// ```
    /// use x_map::maps::CIndexMap;
    /// let mut map = CIndexMap::new();
    /// map.insert("foo", "bar").unwrap();
    ///
    /// // Prints `map.get("foo").unwrap() = "bar"`
    /// dbg!(map.get("foo").unwrap());
    ///
    /// // Prints `map.get_no_peq("foo").unwrap() = "bar"`
    /// dbg!(map.get_no_peq("foo").unwrap());
    /// ```
    ///
    ///
    /// # Incorrect usage
    /// ```
    /// use x_map::maps::CIndexMap;
    /// let string_one = String::from("foo");
    /// let string_two = String::from("bar");
    ///
    /// let mut map = CIndexMap::new();
    /// map.insert(string_one.to_string(), string_two.to_string()).unwrap();
    ///
    ///
    /// // Prints `map.get(string_one.to_string()) = Some("bar")`
    /// dbg!(map.get(string_one.to_string()));
    ///
    /// // Prints `map.get_no_peq(string_one.to_string()) = None`
    /// dbg!(map.get_no_peq(string_one.to_string()));
    /// ```
    pub fn get_no_peq(&self, key: K) -> Option<&V> {
        let key_ptr = ptr::from_ref(&key);
        let mut i = 0;

        // SAFETY:
        // We only iterate the memory space between 0 and self.pos, which is always initialized.
        //
        // The type system guarantees that the supplied `key`, the position of the pointer, and the
        // `size_of::<K>` are all already valid. This means `mem_cmp` can safely be called,
        // as the data in `self.keys[0..self.pos]` is already initialized, and is valid for both
        // the size of `key`, and the size of type `K`.
        unsafe {
            while i <= self.pos {
                let cmp = mem_cmp(key_ptr as *const u8, self.keys.offset(i) as *const u8, size_of::<K>());
                match cmp {
                    None => { return Some(&*self.values.offset(i)); }
                    Some(_) => {}
                }

                i += 1;
            }
        }

        None
    }
}

impl<K: PartialEq, V> CIndexMap<K, V> {
    /// Gets the value associated with the specified key.
    ///
    /// Multiple implementations exist for `get`:
    /// - When `K` does not implement `PartialEq`, `get` will perform bytewise comparison between
    ///   the input and the map keys. This is a fallback implementation, and will not always produce
    ///   expected results. This implementation will have unexpected results on dynamically sized
    ///   types, such as `String` and `Vec`.
    ///
    /// - When `K` implements `PartialEq`, `get` will call `PartialEq::partial_eq()` for comparison
    ///   between the input and the map keys. This is the preferred implementation, and will produce
    ///   the expected results.
    pub fn get(&self, key: K) -> Option<&V> {
        // SAFETY:
        // We know that all elements from (self.keys + 0) to (self.keys + self.pos) are initialized.
        // Thus, reading from memory for each allocation of size_of::<K> is correct.
        unsafe {
            for i in 0..(self.pos + 1) {
                if *self.keys.add(i as usize) == key {
                    return Some(&*self.values.add(i as usize));
                }
            }
        };

        None
    }

    /// Checks if the map contains the provided key.
    ///
    /// - If it does, this function returns `true`,
    /// - If it does not, this function returns `false`.
    pub fn contains_key(&self, key: K) -> bool {
        // SAFETY:
        // We only iterate over the data between 0 and self.pos, which is always initialized.
        unsafe {
            for i in 0..(self.pos + 1) {
                if *self.keys.add(i as usize) == key {
                    return true;
                }
            }
        };

        false
    }
}

impl<K, V: PartialEq> CIndexMap<K, V> {
    /// Checks if the map contains the provided value.
    ///
    /// - If it does, this function returns `true`,
    /// - If it does not, this function returns `false`.
    pub fn contains_value(&self, value: V) -> bool {
        // SAFETY:
        // We only iterate over the data between 0 and self.pos, which is always initialized.
        unsafe {
            for i in 0..(self.pos + 1) {
                if *self.values.add(i as usize) == value {
                    return true;
                }
            }
        };

        false
    }
}