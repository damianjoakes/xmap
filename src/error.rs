//! Module containing code for various errors that may need to be handled.

use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Write};

pub enum MapErrorKind {
    AllocationError,
    AccessError
}

/// A struct handling error reporting for the `CIndexMap` type.
///
/// This error contains the kind of error that the map ran into, and the message to display
/// when displaying the error.
pub struct CIndexMapError {
    /// A static string containing the message associated with the error.
    message: &'static str,

    /// A `MapErrorKind`, containing the type of error that the map encountered.
    kind: MapErrorKind
}

impl CIndexMapError {
    /// Constructs a new CIndexMapError.
    ///
    /// This is only used within `x-map`, and cannot be called externally.
    pub(in crate) fn new(
        kind: MapErrorKind,
        message: &'static str
    ) -> CIndexMapError {
        CIndexMapError {
            kind,
            message
        }
    }
}

impl Debug for CIndexMapError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.message)
    }
}

impl Display for CIndexMapError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.message)
    }
}

impl Error for CIndexMapError {}