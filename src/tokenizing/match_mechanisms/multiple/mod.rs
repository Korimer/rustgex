mod exact_counted;
mod indefinite;
use crate::tokenizing::match_mechanisms::matching::{Matchable,Matcher};

pub struct MultipleMatcher<T: MultipleMatchable> {
    matcher: T
}

impl <T: MultipleMatchable> Matchable for MultipleMatcher<T> {
    fn matches(&self, tomatch: &Vec<char>, startind: usize) -> Vec<usize> {
        self.matcher.matches(tomatch, startind)
    }
}

impl <T: MultipleMatchable> Matcher for MultipleMatcher<T> {
    type ExtendsTo = T::ExtendsTo; //tmp to appease compiler
    type ExtendsFrom = T::ExtendsFrom;
    
    fn canextend(&self, chr: &char) -> bool {
        todo!()
    }
    
    fn extend(self, chr: char) -> Self::ExtendsTo {
        todo!()
    }
    
}


trait MultipleMatchable: Matcher {}

