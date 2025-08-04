use std::collections::HashSet;

use super::super::{
    individual::GeneralIndividualMatcher,
    matching::{Extensible, Matchable}, 
};

pub struct SetMatcher {
    closed: bool,
    contents: HashSet<char>,
    negated: bool,
}

impl SetMatcher {
    pub fn new() -> Self {
        Self {closed: false, contents: HashSet::new(), negated: false}
    }
    pub fn from(chrs: &[char]) -> Self {
        let mut to_fill = Self::new();
        to_fill.contents.extend(chrs.iter());
        to_fill.contents.shrink_to_fit();
        to_fill.closed = true;
        to_fill
    }
    pub fn negate(&mut self) {
        self.negated = true;
    }
}

impl Matchable for SetMatcher {
    fn matches(&self, tomatch: &Vec<char>, ind: usize) -> Vec<usize> {
        if !self.closed { panic!("set was never closed")}
        if tomatch.len() <= ind {return vec![];}
        let contains = self.contents.contains(&tomatch[ind]); 
        if contains != self.negated {
            vec![ind+1]
        }
        else {
            vec![]
        }
    }
}

impl Extensible for SetMatcher {
    fn extend(&mut self, chr: char) -> bool {
        if self.closed {false}
        else {
            if chr == ']' {self.closed = true}
            else {self.contents.insert(chr);}
            true
        }
    }
}

impl GeneralIndividualMatcher for SetMatcher {
    fn try_create(chr: char) -> Option<Box<dyn GeneralIndividualMatcher>> where Self: Sized {
        if chr == '[' { Some(Box::new(SetMatcher::new())) }
        else { None }
    }
}