mod exact_counted;
mod indefinite;
mod optional;

use indefinite::IndefiniteMatcher;

use super::matching::{Matchable,Extensible, MorphInto, TryMorph, TokenMorph};

pub struct MultipleMatcher(Box<dyn GeneralMultipleMatcher>);

impl Matchable for MultipleMatcher {
    fn matches(&self, tomatch: &Vec<char>, startind: usize) -> Vec<usize> {
        todo!()
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
        todo!()
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