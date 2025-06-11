use super::matchable::*;

pub struct LiteralMatcher {
    chars: Vec<char>
}
pub struct CountedMatcher {
    literal: LiteralMatcher,
    count: usize
}

impl LiteralMatcher {
    pub fn new(chars: Vec<char>) -> Self {
        LiteralMatcher {chars}
    }
}
impl CountedMatcher {
    pub fn new(chars: Vec<char>, count: usize) -> Self {
        CountedMatcher {
            literal: LiteralMatcher {chars},
            count
        }
    }
}

impl Matchable for LiteralMatcher {
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

//a tuple class is probably the exact tool for the job here
struct TextCord {
    from: usize,
    len: usize
}
impl TextCord {
    //oh well
    fn totup(self) -> (usize,usize) {
        (self.from,self.len)
    }
}