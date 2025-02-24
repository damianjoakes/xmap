//! Contains the type alias for the internal `Result` enum for `x-map`.

use crate::error::CIndexMapError;

/// Type alias for `std::result::Result<T, CIndexMapError>`.
pub type Result<T> = std::result::Result<T, CIndexMapError>;