use crate::utils::charid;
use crate::utils::phantom_matcher::PhantomMatcher;

use crate::tokenizing::match_mechanisms;
use match_mechanisms::multiple;
use match_mechanisms::matching::Matchable;
use match_mechanisms::matching::Matcher;

pub struct Literal<'a> {
    chr: &'a char
}

impl <'a> Matchable for Literal<'a> {
    fn matches(&self, tomatch: &Vec<char>, ind: usize) -> Vec<usize> {
        if tomatch[ind] == *self.chr {
            vec![ind]
        }
        else {
            vec![]
        }
    }
}

// impl <'a> Matcher for Literal<'a> {
//     type ExtendsTo = multiple::MultipleMatcher;
//     type ExtendsFrom = PhantomMatcher;
    
//     fn canextend(&self, chr: &char) -> bool {
//         todo!()
//     }
    
//     fn extend(self, chr: char) -> Self::ExtendsTo {
//         todo!()
//     }
    
//     fn extend_or_return(self, chr: char) -> Result<Self::ExtendsTo,char> where Self: Sized {
//         if !self.canextend(&chr) {
//             Err(chr)
//         }
//         else {
//             Ok(self.extend(chr))
//         }
//     }
// }