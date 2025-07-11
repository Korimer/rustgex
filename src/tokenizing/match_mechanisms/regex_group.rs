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
        let totalchars = tomatch.len();
        let mytokens = self.tokens.len();

        let mut possible_matches = Vec::new();
        for ind in 0..totalchars {
            possible_matches.append(&mut self.tokens[0].matches(tomatch,ind));
        }
        
        for tkn in &self.tokens {
            let mut i = 0;
            while i < possible_matches.len() {
                let matchoutcome = tkn.matches(tomatch, i); 
                if matchoutcome.len() != 0 {
                    possible_matches[i] = matchoutcome[0];
                    i+= 1;
                }
                else {
                    possible_matches.remove(i);
                }
            }
        }
        possible_matches
    }
}