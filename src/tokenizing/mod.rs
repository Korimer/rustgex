pub mod match_mechanisms;
pub mod classifychar;

use match_mechanisms::RegexMatchSequence;
use classifychar::RegExReader;

pub struct Pattern(RegexMatchSequence);

impl Pattern {
    pub fn new(pattern: &str) -> Self {
        let reader = RegExReader::new(pattern);
        Self {
            0: reader.into_pattern()
        }
    }
}