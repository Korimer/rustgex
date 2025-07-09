mod literal;
mod set;
mod special;

use literal::LiteralMatcher;
use set::SetMatcher;
use special::SpecialMatcher;
use super::matching::{Matchable, Extensible};

//technically this should be matcher-maker... but matchmaker is funnier
type MatchMaker = fn(char) -> Option<Box<dyn GeneralIndividualMatcher>>;

const PRECEDENCE: &[MatchMaker] = &[
    SpecialMatcher::try_create,
    SetMatcher::try_create,
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
    fn canextend(&self, chr: char) -> bool {
        todo!()
    }

    fn extend(self: Box<Self>, chr: char) -> Box<dyn Extensible> {
        todo!()
    }
}

pub trait GeneralIndividualMatcher: Matchable {
    fn try_create(chr: char) -> Option<Box<dyn GeneralIndividualMatcher>> where Self: Sized;
}

pub mod regex_aliases {
    use crate::tokenizing::match_mechanisms::individual::literal::LiteralMatcher;

    use super::super::token::Token;

    pub enum CHARACTER {
        NewLine,
        Tab,
    }

    pub enum CHARACTERCLASS {
        Negation(Box<CHARACTERCLASS>),
        DecimalDigit,
        WordChar,
        WhiteSpace,
    }
    impl CHARACTER {
        pub fn translate(&self) -> LiteralMatcher {
            LiteralMatcher::new(match *self {
                CHARACTER::NewLine => '\n',
                CHARACTER::Tab => ' ',
            })
        }
    }
}