use std::str::Chars;

use crate::utils::regex_aliases::ParsedChar;

use super::matching::Matchable;

use super::token::Token;

pub struct TokenGroup {
    tokens: Vec<Token>
}

impl TokenGroup {
    pub fn new() -> Self {
        Self {tokens: Vec::new()}
    }
    pub fn from(pattern: Vec<ParsedChar>) -> Self {
        let mut me = Self::new();
        me.init(pattern);
        me
    }
    fn init(&mut self, chars: Vec<ParsedChar>) {
        for chr in chars {
            if let Some(inner) = self.tokens.last_mut() {
                if inner.try_extend(*chr.unwrap_char()) {continue}
            }
            self.tokens.push(Token::new(chr));
        }
    }
}

impl Matchable for TokenGroup {
    fn matches(&self, tomatch: &Vec<char>, startind: usize) -> Vec<usize> {
        self.tokens[0].quickmatch(tomatch)
    }
}