pub mod phantom_matcher {
    use crate::tokenizing::match_mechanisms::matching;
    use matching::Matchable;
    use matching::Matcher;

    pub struct PhantomMatcher {}

    impl Matchable for PhantomMatcher {
        fn matches(&self, tomatch: &Vec<char>, startind: usize) -> Vec<usize> {
            unreachable!()
        }
    }
    impl Matcher for PhantomMatcher {
        type ExtendsFrom = PhantomMatcher;
        type ExtendsTo = PhantomMatcher;
    
        fn canextend(&self, chr: &char) -> bool {
            unimplemented!()
        }
    
        fn extend(self, chr: char) -> Self::ExtendsTo {
            unimplemented!()
        }
    }
}

pub mod charid {
    use std::sync::LazyLock;
    use std::collections::{HashMap,HashSet};
    static SPECIAL_CHARS: LazyLock<HashSet<char>> = LazyLock::new(|| HashSet::from([
            '\\', '^', '$', '.', '|', '?', '*', '+', '(', ')', '[', ']', '{', '}', 
    ]));
    static BRACKETS: LazyLock<HashMap<char,char>> = LazyLock::new(|| HashMap::from([
        ('(',')'),
        ('[',']'),
        ('{','}'),
    ]));

    pub fn is_special(chr: &char) -> bool {
        (*SPECIAL_CHARS).contains(chr)
    }
    pub fn match_bracket(chr: &char) -> Option<char> {
        (*BRACKETS).get(chr).copied()
    }

    pub enum CharAlias {
        OpenCurly,
        ClosedCurly
    }
    impl CharAlias {
        pub fn identify(&self) -> char {
            match self {
                Self::OpenCurly => '{',
                Self::ClosedCurly => '}',
            }
        }
    }
}