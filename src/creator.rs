use crate::{Document, Runner};
use std::error::Error;

//TODO: Change the return tupe of the Modifier, Creator and Filter type into different subtypes and
//enums
pub trait Creator {
    fn create(&mut self, doc: &Document) -> Result<Document, Box<dyn Error>>;
}

impl<T> Runner for T
where
    T: Creator,
{
    fn run(&mut self, doc: &mut Document) -> Result<Continue, Box<dyn Error>> {
        self.filter(doc)
    }
}
