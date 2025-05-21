use crate::matchable::*;
struct Token<T: Matchable> {
    token: Box<T>,
    behavior: i32
}

enum Occurance {
    Once,
    AtLeastOnce {islazy: bool},
    NoneOr(Box<Occurance>)
}

impl <'a,T: Matchable> Matchable for Token<T> {
    fn matchesof(&self, tomatch: &str) -> Vec<(usize,usize)> {
        unimplemented!()
    }
}