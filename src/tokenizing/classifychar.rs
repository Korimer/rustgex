use std::{collections::VecDeque, str::Chars};

use crate::tokenizing::match_mechanisms::{
    exact_counted::ExactCountedMatcher,
    indefinite::IndefiniteMatcher,
    literal::LiteralMatcher,
    meta::MetaMatcher,
    special::SpecialMatcher,
};

pub struct RegExReader<'a> {
    escaped: bool,
    txtiter: Chars<'a>,
    buffer: VecDeque<char>,
    expecting: Option<char>
}

impl <'a> RegExReader<'a> {
    pub fn new(text: &'a str) -> Self {
        RegExReader { 
            escaped: false,
            txtiter: text.chars().into_iter(), 
            buffer: VecDeque::new(),
            expecting: None
        }
    }
    pub fn read(&mut self) -> Option<Tier> {
        if self.escaped {
            self.escaped = false;
            Some(self.literalread())
        }
        else {

            unimplemented!()
        }
    }

    fn literalread(&mut self) -> Tier {
        let chr = self.txtiter.next().unwrap();
        Tier::Literal(LiteralMatcher::new(Vec::from([chr])))
    }

}

enum Tier {
    Literal(LiteralMatcher),
    Special(SpecialMatcher),
    Counted(ExactCountedMatcher),
    Indefinite(IndefiniteMatcher),
    Meta(MetaMatcher) //doesnt really align with the naming scheme...
}