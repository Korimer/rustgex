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
        me.initvia(pattern);
        me
    }
    fn init(&mut self, chars: &mut Chars) {
        while let Some(chr) = chars.next() {
            let wrapped = ParsedChar::Char(chr);
            if let Some(inner) = self.tokens.last_mut() {
                if inner.try_extend(chr) {continue}
            }
            self.tokens.push(Token::new(wrapped));
        }
    }
    fn initvia(&mut self, chars: Vec<ParsedChar>) {
        for chr in chars {
            if let Some(inner) = self.tokens.last_mut() {
                if inner.try_extend(*chr.unwrap_char()) {continue}
            }
        }
    }
}

impl Matchable for TokenGroup {
    fn matches(&self, tomatch: &Vec<char>, startind: usize) -> Vec<usize> {
        self.tokens[0].quickmatch(tomatch)
    }
}