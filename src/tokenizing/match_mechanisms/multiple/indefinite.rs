use crate::tokenizing::match_mechanisms::{self, matching::Extensible, multiple::GeneralMultipleMatcher};
use match_mechanisms::matching::Matchable;

pub struct IndefiniteMatcher(pub Box<dyn Matchable>);

impl Matchable for IndefiniteMatcher {
    fn matches(&self, tomatch: &Vec<char>, startind: usize) -> Vec<usize> {
        let mut ind = startind;
        let mut span = Vec::new();
        loop {
            let mut result = self.0.matches(tomatch, ind);
            if result.is_empty() {break;}
            span.append(&mut result);
            ind += 1;
        }
        span
    }
}

impl Extensible for IndefiniteMatcher {
    fn extend(&mut self, chr: char) -> bool {
        false
    }
}

impl GeneralMultipleMatcher for IndefiniteMatcher {}