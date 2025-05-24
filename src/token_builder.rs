use std::collections::VecDeque;

use crate::classifychar::*;
use crate::matchable::*;


pub struct TokenBuilder {
    tokens: Vec<Box<dyn Matchable>>
}
impl TokenBuilder {
    /*
    Gameplan:

    */
    pub fn new(pattern: &str) -> Self {
        Self::group_text(pattern);
        unimplemented!()
    }

    fn group_text(pattern: &str) -> Vec<Vec<char>> {
        let mut builder = TokenBuilder {
            tokens: Vec::new()
        };
        let mut fullpattern: VecDeque<char> = pattern.chars().into_iter().collect();
        let mut buffer = VecDeque::new();
        let mut classifier = Classifier::new();
        while let Some(chr) = fullpattern.pop_front() {
            match classifier.classify(chr) {
                Chartype::Literal => buffer.push_front(chr),
                Chartype::Grouping => (),
                Chartype::Special => (),
            };
        }
        println!("{:?}",buffer);
        unimplemented!()
    }
}

pub fn popuntil<T: PartialEq>(bound: T, vectopop: &mut Vec<T>) -> Vec<T> {
    let mut leading = VecDeque::new();
    while let Some(val) = vectopop.pop() {
        if val == bound {break;} 
        leading.push_front(val);
    }
    leading.into()
}
