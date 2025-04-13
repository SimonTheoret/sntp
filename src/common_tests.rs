use std::error::Error;

#[derive(derive_more::Display, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DummyError(pub &'static str);

impl Error for DummyError{}
