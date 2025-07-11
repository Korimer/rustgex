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

        if possible_matches.len() == 0 {
            println!("DEBUG: No potential matches found.");
        } else {
            println!("DEBUG: Found potential matches at indexes {possible_matches:?}");
        }

        let mut tokennum = 0;
        for tkn in &self.tokens {
            tokennum += 1;
            let mut i = 0;
            while i < possible_matches.len() {
                let matchoutcome = tkn.matches(tomatch, i); 
                if matchoutcome.len() != 0 {
                    println!("\t possible: {}, tokennum: {}, outcome: {}",possible_matches[i],tokennum,matchoutcome[0]);
                    println!("1+{}-{} is {} btw",possible_matches[i],tokennum,possible_matches[i]-tokennum+1);
                    println!("Proceeding with match at index {}, currently on character {}",possible_matches[i]-tokennum+1,matchoutcome[0]);
                    possible_matches[i] = matchoutcome[0];
                    i+= 1;
                }
                else {
                    println!("Match at index {} aborted as it failed to match token {tokennum}",possible_matches[i]-tokennum+1);
                    possible_matches.remove(i);
                }
            }
        }
        possible_matches
    }
}