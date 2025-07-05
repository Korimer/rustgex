use crate::tokenizing::match_mechanisms::individual::{self, GeneralIndividualMatcher, IndividualMatcher};

use super::matching::{Matchable,Extensible};

pub struct Token {
    inner: Box<dyn Extensible>
}

impl Token {
    pub fn new(chr: char) -> Self {
        Self {
            inner: Box::new(
                IndividualMatcher::from(chr)
            )
        }
    }
}

impl Matchable for Token {
    fn matches(&self, tomatch: &Vec<char>, startind: usize) -> Vec<usize> {
        self.inner.matches(tomatch, startind)
    }
}