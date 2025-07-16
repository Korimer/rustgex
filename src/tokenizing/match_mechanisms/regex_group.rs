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
        me.load(pattern);
        me
    }
    pub fn load(&mut self, chars: Vec<ParsedChar>) {
        //AKA, we have streams at home
        let mut pchar_iter = chars.into_iter().peekable();
        let mut iterref= &mut pchar_iter;
        while iterref.peek().is_some() {
            let mut newtkn = Token::new();
            iterref = newtkn.feed(iterref);
            self.tokens.push(newtkn);
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
        }
        
        
        
        let mut tokennum = 1;
        for tkn in (&self.tokens).into_iter().skip(1) {
            tokennum += 1;
            println!("DEBUG: Found potential matches at indexes {possible_matches:?}");
            let mut i = 0;
            while i < possible_matches.len() {
                let matchoutcome = if possible_matches[i] < totalchars {tkn.matches(tomatch, possible_matches[i])} else {vec![]}; 
                if matchoutcome.len() != 0 {
                    println!("Proceeding with match at index {}, currently on character {}",i+1,matchoutcome[0]);
                    possible_matches[i] = matchoutcome[0];
                    i+= 1;
                }
                else {
                    println!("Match at index {} aborted as it failed to match token {tokennum} on character {}",i, possible_matches[i]);
                    possible_matches.remove(i);
                }
            }
        }
        println!("TOKEN_NUM IS {tokennum} compared to token size of {}", self.tokens.len());
        possible_matches
    }
}