use crate::{Continue, Document, Runner};
use std::error::Error;

// pub trait Modifier {
//     fn modify(&mut self, doc: &mut Document) -> Result<Continue, Box<dyn Error>>;
// }
//
// impl<T> Runner for T
// where
//     T: Modifier,
// {
//     fn run(&mut self, doc: &mut Document) -> Result<Continue, Box<dyn Error>> {
//         self.modify(doc)
//     }
// }
