pub trait Matchable {
    fn matchesof(&self, tomatch: &str) -> Vec<(usize,usize)>;
    fn tomatch(self, tomatch: &str) -> MatchOutcome<Self> where Self: Sized {
        MatchOutcome {
            matches: self.matchesof(tomatch),
            matchof: Box::new(self)
        }
    }
}

//This might break if you want to match non-utf-8 chars.
pub fn tochararr(string: &str) -> Vec<char>{
    string.chars().into_iter().collect()
}

pub struct MatchOutcome<T: Matchable> {
    matches: Vec<(usize,usize)>,
    matchof: Box<T>
}

impl <T: Matchable> MatchOutcome<T> {
    fn nextmatch(&mut self) -> Option<(usize,usize)> {
        self.matches.pop()
    }
}
