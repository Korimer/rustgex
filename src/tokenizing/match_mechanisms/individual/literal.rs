use super::super::{
    individual::GeneralIndividualMatcher,
    matching::{Extensible, Matchable}, 
};

pub struct LiteralMatcher {
    chr: char
}

impl LiteralMatcher {
    pub fn new(chr: char) -> Self {
        Self {chr}
    }
}

impl Matchable for LiteralMatcher {
    fn matches(&self, tomatch: &Vec<char>, ind: usize) -> Vec<usize> {
        if tomatch[ind] == self.chr {
            vec![ind]
        }
        else {
            vec![]
        }
    }
}

impl Extensible for LiteralMatcher {
    fn canextend(&self, chr: &char) -> bool {
        todo!()
    }
    
    fn extend(self, chr: char) -> Box<dyn Extensible> {
        todo!()
    }
}

impl GeneralIndividualMatcher for LiteralMatcher {
    fn try_create(chr: char) -> Option<Box<dyn GeneralIndividualMatcher>> where Self: Sized {
        todo!()
    }
}