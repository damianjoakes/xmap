# xmap

**NOTE: This crate is a work in progress. As time goes on, more features may 
be added or removed.**

# About
## What is xmap?
xmap is a crate containing various new vector/map implementations, including multi-indexed maps.

## What does this include?
- `CIndexMap<K, V>` - A chronologically-indexed map. Allows accessing elements by both insertion
order and by key. 


# Types of maps
## `CIndexMap<K, V>`
### Purpose
A key/value map designed to prioritize insertion order (chronological-index-map).

All keys in this map are accessible directly by their insertion order. All values in this map are 
accessible directly by their associated key.
