use crate::collector::StatsCollector;
use crate::{Continue, Document, Runner};
use std::error::Error;
use unicode_segmentation::UnicodeSegmentation;

pub trait Filter {
    fn filter(&mut self, doc: &Document) -> Result<Continue, Box<dyn Error>>;
}

impl<T> Runner for T
where
    T: Filter,
{
    fn run(&mut self, doc: &mut Document) -> Result<Continue, Box<dyn Error>> {
        self.filter(doc)
    }
}

pub struct MinCharsLengthFilter(pub usize);

impl Filter for MinCharsLengthFilter {
    fn filter(&mut self, doc: &Document) -> Result<Continue, Box<dyn Error>> {
        let len = doc.text.chars().count();
        Ok(len >= self.0)
    }
}

pub struct MaxCharsLengthFilter(pub usize);

impl Filter for MaxCharsLengthFilter {
    fn filter(&mut self, doc: &Document) -> Result<Continue, Box<dyn Error>> {
        let len = doc.text.chars().count();
        Ok(len <= self.0)
    }
}

pub struct MinLengthFilter(pub usize);

impl Filter for MinLengthFilter {
    fn filter(&mut self, doc: &Document) -> Result<Continue, Box<dyn Error>> {
        let len = doc.text.len();
        Ok(len >= self.0)
    }
}

pub struct MaxLengthFilter(pub usize);

impl Filter for MaxLengthFilter {
    fn filter(&mut self, doc: &Document) -> Result<Continue, Box<dyn Error>> {
        let len = doc.text.len();
        Ok(len <= self.0)
    }
}

pub struct MinUnicodeLengthFilter(pub usize);

impl Filter for MinUnicodeLengthFilter {
    fn filter(&mut self, doc: &Document) -> Result<Continue, Box<dyn Error>> {
        let len = doc.text.graphemes(true).count();
        Ok(len >= self.0)
    }
}

pub struct MaxUnicodeLengthFilter(pub usize);

impl Filter for MaxUnicodeLengthFilter {
    fn filter(&mut self, doc: &Document) -> Result<Continue, Box<dyn Error>> {
        let len = doc.text.graphemes(true).count();
        Ok(len <= self.0)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::Pipeline;

    fn build_simple_documents() -> Vec<Document> {
        vec![
            Document::from(String::from("This is dummy longer text")),
            Document::from(String::from("This is dummy shorter")),
            Document::from(String::from("much shorter")),
            Document::from(String::from("s!")),
            Document::from(String::from("")),
        ]
    }

    fn build_max_chars_modifier_pipeline() -> Pipeline<Vec<Document>> {
        let source = build_simple_documents();
        let modifier = Box::new(MaxCharsLengthFilter(12));
        let seq: Vec<Box<dyn Runner>> = vec![modifier];
        Pipeline::new(seq, source)
    }

    fn build_max_length_modifier_pipeline() -> Pipeline<Vec<Document>> {
        let source = build_simple_documents();
        let modifier = Box::new(MaxLengthFilter(12));
        let seq: Vec<Box<dyn Runner>> = vec![modifier];
        Pipeline::new(seq, source)
    }

    fn build_max_unicode_modifier_pipeline() -> Pipeline<Vec<Document>> {
        let source = build_simple_documents();
        let modifier = Box::new(MaxUnicodeLengthFilter(12));
        let seq: Vec<Box<dyn Runner>> = vec![modifier];
        Pipeline::new(seq, source)
    }

    fn build_min_chars_modifier_pipeline() -> Pipeline<Vec<Document>> {
        let source = build_simple_documents();
        let modifier = Box::new(MinCharsLengthFilter(12));
        let seq: Vec<Box<dyn Runner>> = vec![modifier];
        Pipeline::new(seq, source)
    }

    fn build_min_unicode_modifier_pipeline() -> Pipeline<Vec<Document>> {
        let source = build_simple_documents();
        let modifier = Box::new(MinUnicodeLengthFilter(12));
        let seq: Vec<Box<dyn Runner>> = vec![modifier];
        Pipeline::new(seq, source)
    }

    fn build_min_length_modifier_pipeline() -> Pipeline<Vec<Document>> {
        let source = build_simple_documents();
        let modifier = Box::new(MinLengthFilter(12));
        let seq: Vec<Box<dyn Runner>> = vec![modifier];
        Pipeline::new(seq, source)
    }

    #[test]
    fn test_modifier_max_length_chars() {
        let pipeline = build_max_chars_modifier_pipeline();
        let (mut src, mut runners) = pipeline.into();
        for (idx, exemple) in src.iter_mut().enumerate() {
            for runner in runners.iter_mut() {
                let c = runner.run(exemple).unwrap();
                if idx >= 2 {
                    dbg!(idx);
                    dbg!(c);
                    assert!(c)
                } else {
                    dbg!(idx);
                    dbg!(c);
                    assert!(!c)
                }
            }
        }
    }
    #[test]
    fn test_modifier_max_length_unicode() {
        let pipeline = build_max_unicode_modifier_pipeline();
        let (mut src, mut runners) = pipeline.into();
        for (idx, exemple) in src.iter_mut().enumerate() {
            for runner in runners.iter_mut() {
                let c = runner.run(exemple).unwrap();
                if idx >= 2 {
                    dbg!(idx);
                    dbg!(c);
                    assert!(c)
                } else {
                    dbg!(idx);
                    dbg!(c);
                    assert!(!c)
                }
            }
        }
    }
    #[test]
    fn test_modifier_max_length() {
        let pipeline = build_max_length_modifier_pipeline();
        let (mut src, mut runners) = pipeline.into();
        for (idx, exemple) in src.iter_mut().enumerate() {
            for runner in runners.iter_mut() {
                let c = runner.run(exemple).unwrap();
                if idx >= 2 {
                    dbg!(idx);
                    dbg!(c);
                    assert!(c)
                } else {
                    dbg!(idx);
                    dbg!(c);
                    assert!(!c)
                }
            }
        }
    }
    #[test]
    fn test_modifier_min_length_chars() {
        let pipeline = build_min_chars_modifier_pipeline();
        let (mut src, mut runners) = pipeline.into();
        for (idx, exemple) in src.iter_mut().enumerate() {
            for runner in runners.iter_mut() {
                let c = runner.run(exemple).unwrap();
                if idx <= 2 {
                    dbg!(idx);
                    dbg!(c);
                    assert!(c)
                } else {
                    dbg!(idx);
                    dbg!(c);
                    assert!(!c)
                }
            }
        }
    }
    #[test]
    fn test_modifier_min_lenth_unicode() {
        let pipeline = build_min_unicode_modifier_pipeline();
        let (mut src, mut runners) = pipeline.into();
        for (idx, exemple) in src.iter_mut().enumerate() {
            for runner in runners.iter_mut() {
                let c = runner.run(exemple).unwrap();
                if idx <= 2 {
                    dbg!(idx);
                    dbg!(c);
                    assert!(c)
                } else {
                    dbg!(idx);
                    dbg!(c);
                    assert!(!c)
                }
            }
        }
    }

    #[test]
    fn test_modifier_min_length() {
        let pipeline = build_min_length_modifier_pipeline();
        let (mut src, mut runners) = pipeline.into();
        for (idx, exemple) in src.iter_mut().enumerate() {
            for runner in runners.iter_mut() {
                let c = runner.run(exemple).unwrap();
                if idx <= 2 {
                    dbg!(idx);
                    dbg!(c);
                    assert!(c)
                } else {
                    dbg!(idx);
                    dbg!(c);
                    assert!(!c)
                }
            }
        }
    }
}
