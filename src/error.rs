use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub enum MapErrorKind {
    KeyAlreadyExists
}

pub struct MapError {

}

impl Debug for MapError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for MapError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for MapError {

}