use crate::matchable::{self, *};
pub struct Literal {
    chars: Vec<char>
}

impl Literal {
    pub fn new(string: &str) -> Self {
        Literal {
            chars: matchable::tochararr(string)
        }
    }
}

struct Token<T: Matchable> {
    token: Box<T>,
    behavior: i32
}

enum Occurance {
    Once,
    AtLeastOnce {islazy: bool},
    NoneOr(Box<Occurance>)
}

impl <'a,T: Matchable> Matchable for Token<T> {
    fn matchesof(&self, tomatch: &str) -> Vec<(usize,usize)> {
        unimplemented!()
    }
}

impl Matchable for Literal {
    fn matchesof(&self, tomatch: &str) -> Vec<(usize,usize)> {
        let textchars = tochararr(tomatch);
        let mut completed = Vec::<(usize,usize)>::new();
        let mut tenative = Vec::<TextCord>::new();
        for i in 0..tomatch.len() {
            tenative.push(TextCord{from: i, len: 0});
            
            let mut j = 0; 
            while j < tenative.len() {
                let mut incr = true;
                let pos = &mut tenative[j];
                let start = pos.from;
                let cur = &mut pos.len;
                if self.chars[*cur] == textchars[start+*cur] {
                    *cur += 1;
                    if *cur == self.chars.len() {
                        completed.push(tenative.remove(j).totup());
                        incr = false;
                    }
                }
                else {
                    tenative.remove(j);
                    incr = false;
                }
                if incr {j+=1;}
            }
        }
        completed
    }
}

struct TextCord {
    from: usize,
    len: usize
}
impl TextCord {
    fn totup(self) -> (usize,usize) {
        (self.from,self.len)
    }
}