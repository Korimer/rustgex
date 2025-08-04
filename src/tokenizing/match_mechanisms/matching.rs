pub trait Matchable {
    //could be renamed to matchesof
    fn matches(&self, tomatch: &Vec<char>, startind: usize) -> Vec<usize>;
    fn quickmatch(&self,tomatch: &Vec<char>) -> Vec<usize> {
        self.matches(tomatch, 0)
    }
}

pub trait Extensible: Matchable {
    fn extend(&mut self, chr: char) -> bool;
}

pub struct TryMorph {
    pub canmorph: fn(char) -> bool,
    pub morph:    fn(Box<dyn TokenMorph>, char) -> Box<dyn TokenMorph>
}
impl TryMorph {
    pub fn new<T: MorphInto + 'static>() -> Self {
        Self {
            canmorph: <T as MorphInto>::canmorph,
            morph: Self::definedmorph::<T>
        }
    }
    fn definedmorph<T: MorphInto + 'static>(tomorph: Box<dyn TokenMorph>, chr: char) -> Box<dyn TokenMorph> {
        <T as MorphInto>::morph(tomorph, chr)
    }
}

pub trait TokenMorph: Extensible {
    fn gettarget(&self) -> TryMorph;
    fn canmorph(&self, chr: char) -> bool {
        (self.gettarget().canmorph)(chr)
    }
    fn morph(self: Box<Self>, chr: char) -> Box<dyn TokenMorph> where Self: Sized + 'static{
        let b = self.gettarget();
        let a: Box<dyn TokenMorph> = self as Box<dyn TokenMorph>;
        (b.morph)(a, chr)
    }
}

pub trait MorphInto: TokenMorph {
    fn canmorph(chr: char) -> bool;
    fn morph(tomorph: Box<dyn TokenMorph>, chr: char) -> Box<Self>; 
}

//This might break if you want to match non-utf-8 chars.
pub fn tochararr(string: &str) -> Vec<char>{
    string.chars().into_iter().collect()
}