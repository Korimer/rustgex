mod match_mechanisms;
mod classifychar;

use match_mechanisms::RegexMatchSequence;
use classifychar::RegExReader;

pub struct Pattern(RegexMatchSequence);

impl Pattern {
    pub fn new(pattern: &str) -> Self {
        Self(RegexMatchSequence::new(pattern))
    }
    pub fn quickmatch(&self,tomatch: &str) -> bool {
        self.0.simple_match(tomatch)
    }
}