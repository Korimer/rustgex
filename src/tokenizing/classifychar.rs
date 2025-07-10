use std::{collections::VecDeque, str::Chars};

use super::match_mechanisms::RegexMatchSequence;
use crate::utils::regex_aliases::ParsedChar;

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

    pub fn into_pattern(&self) -> Vec<ParsedChar> {
        let mut patternvec = Vec::new();
        // ugly clone, fix later
        for chr in self.txtiter.clone() {
            patternvec.push(ParsedChar::Char(chr));
        }
        patternvec
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