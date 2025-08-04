use super::super::{
    individual::GeneralIndividualMatcher,
    matching::{Extensible, Matchable}, 
};

pub struct AnyMatcher;

impl Matchable for AnyMatcher {
    fn matches(&self, tomatch: &Vec<char>, ind: usize) -> Vec<usize> {
        if tomatch.len() <= ind && tomatch[ind] != '\n' {
            vec![ind+1]
        }
        else {
            vec![]
        }
    }
}

impl Extensible for AnyMatcher {
    fn extend(&mut self, chr: char) -> bool {
        false
    }
}

impl GeneralIndividualMatcher for AnyMatcher {
    fn try_create(chr: char) -> Option<Box<dyn GeneralIndividualMatcher>> where Self: Sized {
        if chr == '.' {Some(Box::new(AnyMatcher))}
        else          {None}
    }
}