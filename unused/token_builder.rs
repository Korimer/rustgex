use std::collections::VecDeque;

use crate::tokenizing;
use crate::tokenizing::matchable::*;
use tokenizing::classifychar::*;


pub struct TokenBuilder {
    tokens: Vec<Box<dyn Matchable>>
}
impl TokenBuilder {
    /*
    Gameplan:
        vec (buffer) of every char in the string
        upon encountering the beginning of a group, flush the buffer to the stack
        upon encountering the end of a group, pop the stack
            Eventually, the proccess of flushing a buffer should convert that buffer to a token
            Then, popping the stack would mean converting the topmost level of the stack to a token, and adding that token to a 
    */
    pub fn new(pattern: &str) -> Self {
        Self::group_text(pattern);
        unimplemented!()
    }

    fn group_text(pattern: &str) -> Vec<Vec<char>> {
        let mut builder = TokenBuilder {
            tokens: Vec::new()
        };
        let mut classifier = RegExReader::new(pattern);
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
