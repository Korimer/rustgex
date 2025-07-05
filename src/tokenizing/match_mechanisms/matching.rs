pub trait Matchable {
    fn matches(&self, tomatch: &Vec<char>, startind: usize) -> Vec<usize>;
    fn quickmatch(&self,tomatch: &Vec<char>) -> Vec<usize> {
        self.matches(tomatch, 0)
    }
}

pub trait Extensible: Matchable {
    fn canextend(&self, chr: &char) -> bool;
    fn extend(self, chr: char) -> Box<dyn Extensible>;
    fn try_extend(self, chr: char) -> Option<Box<dyn Extensible>> where Self: Sized {
        if !self.canextend(&chr) {
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