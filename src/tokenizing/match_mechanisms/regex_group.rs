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