mod exact_counted;
mod indefinite;
mod optional;

use indefinite::IndefiniteMatcher;

use super::matching::{Matchable,Extensible, MorphInto, TryMorph, TokenMorph};

pub struct MultipleMatcher(Box<dyn GeneralMultipleMatcher>);

impl Matchable for MultipleMatcher {
    fn matches(&self, tomatch: &Vec<char>, startind: usize) -> Vec<usize> {
        self.0.matches(tomatch, startind)
    }
}

impl Extensible for MultipleMatcher {
    fn extend(&mut self, chr: char) -> bool {
        self.0.extend(chr)
    }
}

trait GeneralMultipleMatcher: Extensible {

}

impl TokenMorph for MultipleMatcher {
    fn gettarget(&self) -> TryMorph {
        TryMorph::new::<Self>()
    }
    
    fn dyn_self(self: Box<Self>) -> Box<dyn TokenMorph> {
        self
    }
}

impl MorphInto for MultipleMatcher {
    fn canmorph(chr: char) -> bool {
        match chr {
            '?' | '*' | '+' => true,
            _ => false
        }
    }

    fn morph(tomorph: Box<dyn TokenMorph>, chr: char) -> Box<Self> {
        Box::new(MultipleMatcher(Box::new(match chr {
            '+' => IndefiniteMatcher,
            _ => panic!()
        }(tomorph))))
    }
}