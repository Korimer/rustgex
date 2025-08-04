use super::super::{
    individual::GeneralIndividualMatcher,
    matching::{Extensible, Matchable}, 
};

pub struct LiteralMatcher {
    chr: char
}

impl LiteralMatcher {
    pub fn new(chr: char) -> Self {
        Self {chr}
    }
}

impl Matchable for LiteralMatcher {
    fn matches(&self, tomatch: &Vec<char>, ind: usize) -> Vec<usize> {
        if tomatch.len() > ind && tomatch[ind] == self.chr {
            vec![ind+1]
        }
        else {
            vec![]
        }
    }
}

impl Extensible for LiteralMatcher {
    fn extend(&mut self, chr: char) -> bool {
        false
    }
}

impl GeneralIndividualMatcher for LiteralMatcher {
    fn try_create(chr: char) -> Option<Box<dyn GeneralIndividualMatcher>> where Self: Sized {
        Some(Box::new(LiteralMatcher::new(chr)))
    }
}