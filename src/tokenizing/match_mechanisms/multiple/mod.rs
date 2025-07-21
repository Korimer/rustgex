mod exact_counted;
mod indefinite;
use crate::tokenizing::match_mechanisms::matching::Extensible;

use super::matching::{Matchable,TokenMorph};

pub trait GeneralMultipleMatcher: Extensible {

}

impl TokenMorph for dyn GeneralMultipleMatcher {
    fn canmorph(chr: char) -> bool {
        match chr {
            '?'|'*'|'+' => true,
            _ => false
        }
    }
    
    fn morph(morphfrom: Box<dyn Matchable>, chr: char) -> Box<Self> {
        todo!()
    }
}