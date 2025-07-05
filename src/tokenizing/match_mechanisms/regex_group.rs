use std::str::Chars;

use super::matching::Matchable;

use super::token::Token;

pub struct TokenGroup {
    tokens: Vec<Token>
}

impl TokenGroup {
    pub fn new() -> Self {
        Self {tokens: Vec::new()}
    }
    fn init(&mut self, chars: &mut Chars) {
        while let Some(chr) = chars.next() {
            self.tokens.last_mut();
        }
    }
}

impl Matchable for TokenGroup {
    fn matches(&self, tomatch: &Vec<char>, startind: usize) -> Vec<usize> {
        unimplemented!()
    }
}