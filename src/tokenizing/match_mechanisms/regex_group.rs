use crate::tokenizing::match_mechanisms::matching::TokenMorph;
use crate::utils::regex_aliases::ParsedChar;

use super::individual::IndividualMatcher;
use super::matching::{Matchable};
pub struct TokenGroup {
    tokens: Vec<Token>
}

type Token = Box<dyn TokenMorph>;

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
        let mut chriter = chars.into_iter();
        if self.tokens.is_empty() {
            if let Some(pchr) = chriter.next() {
                self.newtoken(pchr);
            }
        }
        for pchr in chriter {
            let trailing = self.tokens.last_mut().unwrap();
            let mut fed = match pchr {
                    ParsedChar::Alias(_) => true,
                    ParsedChar::Char(chr) => trailing.extend(chr), //this is a copy. oh well..?
            };
            if !fed {
                if pchr.contains_char() && trailing.canmorph(pchr.unwrap_char()) {
                    let tomorph = self.tokens.pop().unwrap();
                    self.tokens.push(tomorph.morph(pchr.unwrap_char()));
                    fed = true;
                }
            }
            if !fed {
                self.newtoken(pchr);
            }
        }
    }
    fn newtoken(&mut self, pchr: ParsedChar) {
        self.tokens.push(Box::new(IndividualMatcher::from(pchr)));
    }

    fn matchfrom(&self, tomatch: &Vec<char>, startind: usize) -> Option<usize> {
        let tknlen = self.tokens.len();
        if tknlen == 0 {return Some(startind);}

        let mut potential_matches: Vec<Vec<usize>> = Vec::new();

        potential_matches.push(self.tokens[0].matches(tomatch, startind));
        let mut token_ind = 1;

        loop {
            // if there are no viable matches from an index, it's time to backtrack.
            while potential_matches.last().is_some_and(|inds| inds.len() == 0) {
                potential_matches.pop();
                token_ind -= 1;
            }
            // if there's nowhere left to backtrack, a match isn't possible!
            if token_ind == 0 {return None;}
            // all this unwrapping is safe because the above two blocks eliminate the possibility of a vec being empty.
            let match_ind = potential_matches.last_mut().unwrap().pop().unwrap(); //lol
            // Every token matches, so the overall pattern does too
            if token_ind == tknlen {return Some(match_ind)} 
            potential_matches.push(self.tokens[token_ind].matches(tomatch, match_ind));
            token_ind += 1;
        }
    }
}

impl Matchable for TokenGroup {
    fn matches(&self, tomatch: &Vec<char>, startind: usize) -> Vec<usize> {
        let mut allmatches = Vec::new();
        let mut i = startind; 
        while i < tomatch.len() {
            if let Some(match_index) = self.matchfrom(tomatch, i) {allmatches.push(match_index)}
            i += 1;
        }
        return allmatches;
    }
}