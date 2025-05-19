pub trait Matchable {
    fn matchesof(&self, tomatch: &str) -> Vec<usize>;
    fn nextmatch(&self) -> &str;
}

struct MatchOutcome<'a> {
    matched: &'a str,
    matches: Vec<usize>
}

// impl MatchOutcome {
//     fn nextmatch 
// }

