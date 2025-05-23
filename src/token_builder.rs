use std::collections::VecDeque;

use crate::classifychar::*;
use crate::matchable::*;


struct TokenBuilder {
    tokens: Vec<Box<dyn Matchable>>
}
impl TokenBuilder {
    fn new(pattern: &str) -> Self {
        let mut fullpattern: VecDeque<char> = pattern.chars().into_iter().collect();
        let mut read = VecDeque::new();
        let mut Classifier = Classifier::new();
        while let Some(chr) = fullpattern.pop_front() {
            read.push_front(chr);
        }
        unimplemented!()
    }
}

pub fn popuntil<T: PartialEq>(bound: T, vectopop: &mut Vec<T>) -> Vec<T> {
    let mut leading = VecDeque::<T>::new();
    while let Some(val) = vectopop.pop() {
        if val == bound {break;} 
        leading.push_front(val);
    }
    leading.into()
}
