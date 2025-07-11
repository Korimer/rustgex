pub trait Matchable {
    //could be renamed to matchesof
    fn matches(&self, tomatch: &Vec<char>, startind: usize) -> Vec<usize>;
    fn quickmatch(&self,tomatch: &Vec<char>) -> Vec<usize> {
        self.matches(tomatch, 0)
    }
}

pub trait Extensible: Matchable {
    fn canextend(&self, chr: char) -> bool;
    fn extend(self: Box<Self>, chr: char) -> Box<dyn Extensible>;
    fn try_extend(self: Box<Self>, chr: char) -> Option<Box<dyn Extensible>>{
        if !self.canextend(chr) {
            None
        }
        else {
            Some(self.extend(chr))
        }
    }
}

//This might break if you want to match non-utf-8 chars.
pub fn tochararr(string: &str) -> Vec<char>{
    string.chars().into_iter().collect()
}