use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Copy, Clone, Debug)]
pub struct InvalidXml;

impl Display for InvalidXml {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Invalid xml file")
    }
}

impl Error for InvalidXml {}

#[derive(Copy, Clone, Debug)]
pub struct MissingNode(pub usize);

impl Display for MissingNode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let MissingNode(a) = self;
        write!(f, "Missing node {}", a)
    }
}

impl Error for MissingNode {}

#[derive(Copy, Clone, Debug)]
pub struct InvalidOwnedData;

impl Display for InvalidOwnedData {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Node contains wrong data type")
    }
}

impl Error for InvalidOwnedData {}