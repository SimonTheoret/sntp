use ahash::AHashMap;
use std::any::Any;
use std::error::Error;
use std::fmt::Debug;

use uuid::Uuid;

mod collector;
mod creator;
mod filter;
mod modifier;

#[cfg(test)]
mod common_tests;

//TODO: Change the return tupe of the Modifier, Creator and Filter type into different subtypes and
//enums

#[derive(derive_more::Deref, derive_more::DerefMut, Debug)]
struct AnyMap(AHashMap<&'static str, Box<dyn Any>>);

struct Pipeline<T>
where
    T: IntoIterator<Item = Document>,
{
    source: T,
    sequences: Vec<Box<dyn Runner>>,
}

impl<T> From<Pipeline<T>> for (T, Vec<Box<dyn Runner>>)
where
    T: IntoIterator<Item = Document>,
{
    fn from(val: Pipeline<T>) -> Self {
        (val.source, val.sequences)
    }
}

#[derive(Debug, derive_more::Deref, derive_more::DerefMut, derive_more::Display)]
pub struct DocID(Uuid);

impl Default for DocID {
    fn default() -> Self {
        DocID(Uuid::new_v4())
    }
}

pub struct Document {
    pub text: String,
    id: DocID,
}

impl From<String> for Document {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl Document {
    pub fn new(text: String) -> Self {
        let id = DocID::default();
        Self { id, text }
    }
}

type Continue = bool;

trait Runner {
    /// Execute the run and and tells the loop if we continue or not;
    fn run(&mut self, doc: &mut Document) -> Result<Continue, Box<dyn Error>>;
}

impl<T> Pipeline<T>
where
    T: IntoIterator<Item = Document> + Default,
{
    fn new(seq: Vec<Box<dyn Runner>>, source: T) -> Self {
        Self {
            source,
            sequences: seq,
        }
    }
}

#[cfg(test)]
mod tests {
    use self::common_tests::DummyError;

    use super::*;

    struct DummyOkRunner {}

    #[allow(unused_variables)]
    impl Runner for DummyOkRunner {
        fn run(&mut self, doc: &mut Document) -> Result<bool, Box<dyn Error>> {
            Ok(true)
        }
    }

    struct DummyErrRunner {}

    #[allow(unused_variables)]
    impl Runner for DummyErrRunner {
        fn run(&mut self, doc: &mut Document) -> Result<bool, Box<dyn Error>> {
            Err(Box::new(DummyError("DummyError")))
        }
    }

    fn build_simple_documents() -> Vec<Document> {
        vec![
            Document::from(String::from("This is dummy longer text")),
            Document::from(String::from("This is dummy shorter")),
            Document::from(String::from("much shorter")),
            Document::from(String::from("s!")),
            Document::from(String::from("")),
        ]
    }

    fn build_dummy_pipeline() -> Pipeline<Vec<Document>> {
        let source = build_simple_documents();
        let seq: Vec<Box<dyn Runner>> =
            vec![Box::new(DummyOkRunner {}), Box::new(DummyErrRunner {})];
        Pipeline::new(seq, source)
    }

    #[test]
    fn test_run_pipeline_sequences() {
        let pipeline = build_dummy_pipeline();
        let text = String::from("This is a full document");
        let mut doc = Document::new(text);
        for mut runner in pipeline.sequences {
            let _ = runner.run(&mut doc);
        }
    }

    #[test]
    fn iterate_over_simple_examples() {
        let pipeline = build_dummy_pipeline();
        let (mut src, mut runners) = pipeline.into();
        for exemple in src.iter_mut() {
            for runner in runners.iter_mut() {
                let _ = runner.run(exemple);
            }
        }
    }
}
