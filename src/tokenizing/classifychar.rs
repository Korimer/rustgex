use std::{collections::VecDeque, str::Chars};

use crate::tokenizing::match_mechanisms;
use match_mechanisms::matchable::Matchable;
use match_mechanisms::{
    individual,
    multiple,
    behavioral,
};

pub struct RegExReader<'a> {
    escaped: bool,
    txtiter: Chars<'a>,
    buffer: VecDeque<char>,
    expecting: Option<Tier>
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
    fn read(&mut self) -> Option<char> {
        if self.escaped {
            self.escaped = false;
            Some(self.literalread())
        }
        else {

            unimplemented!()
        }
    }

    fn cur_head(&self) -> Tier {
        unimplemented!()
    }

    fn literalread(&mut self) -> char {
        self.txtiter.next().unwrap()
    }
}

enum Tier {}