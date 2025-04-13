use ahash::AHashMap;
use std::{cell::RefCell, error::Error, panic, sync::OnceLock};



// pub struct PipelineStatsCollector {
//     text_stats: 
// }

#[derive(
    derive_more::Deref,
    derive_more::DerefMut,
    derive_more::Display,
    Debug,
    Clone,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
pub struct ErrorMessage(String);
#[derive(derive_more::Deref, derive_more::DerefMut, Debug, Default)]
pub struct ErrorMap(AHashMap<ErrorMessage, usize>);

impl ErrorMap {
    fn add_err(&mut self, err_msg: ErrorMessage) {
        self.entry(err_msg).and_modify(|v| *v += 1).or_insert(1);
    }
}

pub enum ErrorCollector {
    StopAll,
    StopCurrent,
    Collect(ErrorMap),
}

#[derive(Debug, thiserror::Error)]
#[error("Could not convert an ErrorCollector into a ErrorMap")]
pub struct ErrorCollectorConversionError;

impl TryInto<ErrorMap> for ErrorCollector {
    type Error = ErrorCollectorConversionError;
    fn try_into(self) -> Result<ErrorMap, Self::Error> {
        match self {
            ErrorCollector::StopAll => Err(ErrorCollectorConversionError),
            ErrorCollector::StopCurrent => Err(ErrorCollectorConversionError),
            ErrorCollector::Collect(em) => Ok(em),
        }
    }
}

impl ErrorCollector {
    fn collect_err(&mut self, err: Box<dyn Error>) {
        match self {
            ErrorCollector::StopAll => std::process::exit(1),
            ErrorCollector::StopCurrent => panic!("{}", err.to_string()),
            ErrorCollector::Collect(em) => {
                let err_msg: ErrorMessage = ErrorMessage(err.to_string());
                em.add_err(err_msg)
            }
        };
    }
}

pub trait StatsCollector {
    fn collect_step(&mut self, stats: &mut PipelineStatsCollector, errors: &mut ErrorCollector);
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common_tests::DummyError;

    impl ErrorCollector {
        fn test_len(&self) -> usize {
            match self {
                ErrorCollector::StopAll => 0,
                ErrorCollector::StopCurrent => 0,
                ErrorCollector::Collect(em) => em.len(),
            }
        }
    }

    #[test]
    fn test_error_collector_different_content() {
        let mut err_collector = ErrorCollector::Collect(ErrorMap::default());
        err_collector.collect_err(Box::new(DummyError("This is a dummy")));
        err_collector.collect_err(Box::new(DummyError("This is another dummy")));
        let em: ErrorMap = err_collector.try_into().unwrap();
        dbg!(&em);
        assert_eq!(em[&ErrorMessage(String::from("This is a dummy"))], 1);
        assert_eq!(em[&ErrorMessage(String::from("This is another dummy"))], 1);
    }
    fn test_error_collector_len() {
        let mut err_collector = ErrorCollector::Collect(ErrorMap::default());
        err_collector.collect_err(Box::new(DummyError("This is a dummy")));
        err_collector.collect_err(Box::new(DummyError("This is another dummy")));
        let actual_len = err_collector.test_len();
        assert_eq!(actual_len, 2)
    }
}
