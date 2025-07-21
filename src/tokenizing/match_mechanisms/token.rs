use crate::tokenizing::match_mechanisms::individual::IndividualMatcher;
use crate::utils::regex_aliases::ParsedChar;
use super::matching::{Matchable,Extensible};

use std::iter::Peekable;

pub struct Token {
    inner: Option<Box<dyn Extensible>>
}

impl Token {
    pub fn new() -> Self {
        Self {
            inner: None
        }
    }
    fn feed_single(&mut self, pchr: ParsedChar) {
        if self.inner.is_none() {self.inner = Some(Box::new(IndividualMatcher::from(pchr)));}
        else {
            let tmpinner = std::mem::take(&mut self.inner).unwrap();
            self.inner = Some(tmpinner.extend(pchr.unwrap_char()));
        }
    }
    fn can_feed(&self, pchr: &ParsedChar) -> bool {
        if self.inner.is_none() {true}
        else if pchr.contains_char() {
            let res = self.inner.as_ref().unwrap().canextend(pchr.unwrap_char());
            res
        }
        else {false}
    }
    pub fn feed<'a,I>(&mut self, pchar_iter: &'a mut Peekable<I>) -> &'a mut Peekable<I> 
        where I: Iterator<Item=ParsedChar> 
        {
        while pchar_iter.peek().is_some_and(|itm| self.can_feed(itm)) {
            self.feed_single(pchar_iter.next().unwrap());
        }
        pchar_iter
    }
}
// this is kinda insane. I should implement in Extensible a method that returns itself if it fails to extend.


impl Matchable for Token {
    fn matches(&self, tomatch: &Vec<char>, startind: usize) -> Vec<usize> {
        self.inner.as_ref().unwrap().matches(tomatch, startind)
    }
}