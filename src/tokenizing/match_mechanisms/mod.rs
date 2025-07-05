pub mod behavioral;
pub mod individual;
pub mod multiple;

pub mod matching;
pub mod token;


mod regex_group;
use matching::Matchable;
pub struct RegexMatchSequence(regex_group::TokenGroup);

impl RegexMatchSequence {
    pub fn simple_match(&self, to_match: &str) -> bool {
        self.0.matches(&matching::tochararr(to_match),0).len() != 0
    }
}