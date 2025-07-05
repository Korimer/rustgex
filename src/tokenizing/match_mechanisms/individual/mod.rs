mod literal;
mod special;

use literal::LiteralMatcher;
use special::SpecialMatcher;
use super::matching::{Matchable, Extensible};

type MatchMaker = fn(char) -> Option<Box<dyn GeneralIndividualMatcher>>;

const PRECEDENCE: [MatchMaker; 2] = [
    SpecialMatcher::try_create,
    LiteralMatcher::try_create,
];

pub struct IndividualMatcher(Box<dyn GeneralIndividualMatcher>);

impl IndividualMatcher {
    pub fn from (chr: char) -> IndividualMatcher {
        for matchertype in PRECEDENCE.iter() {
            if let Some(matcher) = matchertype(chr) {
                return IndividualMatcher(matcher);
            }
        }
        unreachable!()
    }
}

impl Matchable for IndividualMatcher {
    fn matches(&self, tomatch: &Vec<char>, startind: usize) -> Vec<usize> {
        (*self.0).matches(tomatch,startind)
    }
}

impl Extensible for IndividualMatcher {
    fn canextend(&self, chr: &char) -> bool {
        todo!()
    }

    fn extend(self, chr: char) -> Box<dyn Extensible> {
        todo!()
    }
}

pub trait GeneralIndividualMatcher: Matchable {
    fn try_create(chr: char) -> Option<Box<dyn GeneralIndividualMatcher>> where Self: Sized;
}