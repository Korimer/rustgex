use crate::matchable::{self, *};
struct Literal<'a> {
    chars: &'a str
}

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
    fn matchesof(&self, tomatch: &str) -> Vec<usize> {
        unimplemented!()
    }
    fn nextmatch(&self) -> &str {
        unimplemented!()
    }
    
}

impl <'a> Matchable for Literal<'a> {
    fn matchesof(&self, tomatch: &str) -> Vec<usize> {
        unimplemented!()   
    }
    fn nextmatch(&self) -> &str {
        unimplemented!()
    }
}