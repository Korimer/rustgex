use crate::matchable::{self, *};
struct Literal {
    chars: Vec<char>
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

impl <'a> Matchable for Literal {
    fn matchesof(&self, tomatch: &str) -> Vec<(usize,usize)> {
        let textchars = tochararr(tomatch);
        let mut completed = Vec::<(usize,usize)>::new();
        let mut tenative = Vec::<TextCord>::new();
        for i in 0..tomatch.len() {
            tenative.push(TextCord{from: i, to: i});
            
            let mut j = 0; 
            while j < self.chars.len() {
                let pos = &mut tenative[j];
                let start = pos.from;
                let cur = &mut pos.to;
                if self.chars[*cur] == textchars[start+*cur] {
                    *cur += 1;
                    if *cur == self.chars.len() {
                        completed.push(tenative.remove(j).totup());
                        j-=1;
                    }
                }
                else {
                    tenative.remove(j);
                    j-=1;
                }
                j+=1;
            }
        }
        completed
    }
}

struct TextCord {
    from: usize,
    to: usize
}
impl TextCord {
    fn totup(self) -> (usize,usize) {
        (self.from,self.to)
    }
}