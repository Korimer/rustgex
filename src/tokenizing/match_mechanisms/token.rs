use super::matching::Matchable;

pub struct Token {
    inner: Box<dyn Matchable>
}

impl Matchable for Token { 
    fn matches(&self, tomatch: &Vec<char>, startind: usize) -> Vec<usize> {
        self.inner.matches(tomatch, startind)
    }
}