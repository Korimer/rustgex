pub trait Matchable {
    fn matches(&self, tomatch: &Vec<char>, startind: usize) -> Vec<usize>;
    fn quickmatch(&self,tomatch: &Vec<char>) -> Vec<usize> {
        self.matches(tomatch, 0)
    }
}

pub trait Matcher: Matchable {
    type ExtendsFrom: Matchable;
    type ExtendsTo: Matchable;

    fn canextend(&self, chr: &char) -> bool;
    fn extend(self, chr: char) -> Self::ExtendsTo;
    fn extend_or_return(self, chr: char) -> Result<Self::ExtendsTo,char> where Self: Sized {
        if !self.canextend(&chr) {
            Err(chr)
        }
        else {
            Ok(self.extend(chr))
        }
    }
}

//This might break if you want to match non-utf-8 chars.
pub fn tochararr(string: &str) -> Vec<char>{
    string.chars().into_iter().collect()
}