use crate::tokenizing::match_mechanisms::individual::{self, GeneralIndividualMatcher, IndividualMatcher};

use super::matching::{Matchable,Extensible};

pub struct Token {
    inner: Option<Box<dyn Extensible>>
}

impl Token {
    fn empty() -> Self {
        Self {
            inner: None
        }
    }
    pub fn new(chr: char) -> Self {
        Self {
            inner: Some(Box::new(
                IndividualMatcher::from(chr)
            ))
        }
    }
    pub fn try_extend(&mut self, chr: char) -> bool {
        if let Some(trueinner) = std::mem::take(&mut self.inner) {
            if trueinner.canextend(chr) {
                self.inner = Some(trueinner.extend(chr));
                return true;
            } else {
                self.inner = Some(trueinner);
            }
        }
        false
    }
}
// this is kinda insane. I should implement in Extensible a method that returns itself if it fails to extend.


impl Matchable for Token {
    fn matches(&self, tomatch: &Vec<char>, startind: usize) -> Vec<usize> {
        self.inner.as_ref().unwrap().matches(tomatch, startind);
    }
}