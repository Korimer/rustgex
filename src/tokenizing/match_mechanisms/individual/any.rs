use super::super::{
    individual::GeneralIndividualMatcher,
    matching::{Extensible, Matchable}, 
};

pub struct AnyMatcher;

impl Matchable for AnyMatcher {
    fn matches(&self, tomatch: &Vec<char>, startind: usize) -> Vec<usize> {
        vec![startind+1]
    }
}

impl Extensible for AnyMatcher {
    fn canextend(&self, chr: char) -> bool {
        todo!()
    }

    fn extend(self: Box<Self>, chr: char) -> Box<dyn Extensible> {
        todo!()
    }
}

impl GeneralIndividualMatcher for AnyMatcher {
    fn try_create(chr: char) -> Option<Box<dyn GeneralIndividualMatcher>> where Self: Sized {
        if chr == '.' {Some(Box::new(AnyMatcher))}
        else          {None}
    }
}