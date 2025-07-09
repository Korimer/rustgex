use std::sync::LazyLock;
use std::collections::hash_set::HashSet;

use super::super::{
    individual::GeneralIndividualMatcher,
    matching::{Extensible, Matchable}, 
};

static SPECIAL_MATCHERS: LazyLock<HashSet<char>> = LazyLock::new(|| HashSet::from([
    '.'
]));

pub struct SpecialMatcher;

impl Matchable for SpecialMatcher {
    fn matches(&self, tomatch: &Vec<char>, startind: usize) -> Vec<usize> {
        todo!()
    }
}

impl Extensible for SpecialMatcher {
    fn canextend(&self, chr: char) -> bool {
        todo!()
    }

    fn extend(self: Box<Self>, chr: char) -> Box<dyn Extensible> {
        todo!()
    }
}

impl GeneralIndividualMatcher for SpecialMatcher {
    fn try_create(chr: char) -> Option<Box<dyn GeneralIndividualMatcher>> where Self: Sized {
        todo!()
    }
}