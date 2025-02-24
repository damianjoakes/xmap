# xmap

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