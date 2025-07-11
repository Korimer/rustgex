mod behavioral;
mod individual;
mod multiple;

mod matching;
mod token;
mod regex_group;

use matching::Matchable;
use crate::utils::{
    regex_reader::RegExReader,
    regex_aliases::ParsedChar
};

use regex_group::TokenGroup;
pub struct RegexMatchSequence(TokenGroup);

impl RegexMatchSequence {
    pub fn new(pattern: &str) -> Self {
        Self::construct(RegExReader::new(pattern))
    }
    
    fn construct(ptn: Vec<ParsedChar>) -> Self {
        Self(TokenGroup::from(ptn))
    }

    pub fn simple_match(&self, to_match: &str) -> bool {
        self.0.matches(&matching::tochararr(to_match),0).len() != 0
    }
}