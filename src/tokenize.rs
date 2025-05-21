use crate::matchable::*;
struct Token<T: Matchable> {
    token: Box<T>,
    behavior: i32
}


/*
Matching thoughts:
More(x) = x occurances, x=0 denotes as many as possible
: (.)    = One Occurance
: (.{x}) = More(x) Occurance
: (.+)   = More(0) Occurance
: (.*)   = Optional More(0) Occurance
*/

enum Occurance {
    One,
    More(usize)
}
struct TokenOccurance <T: Matchable> {
    islazy: bool,
    rate: Occurance,
    token: T
}
struct OptionalOccurance <T: Matchable> {
    token: TokenOccurance<T>
}

impl <'a,T: Matchable> Matchable for Token<T> {
    fn matchesof(&self, tomatch: &str) -> Vec<(usize,usize)> {
        unimplemented!()
    }
}