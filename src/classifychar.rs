pub struct Classifier {
    escaped: bool
}

impl Classifier {
    pub fn new() -> Self {
        Classifier{escaped: false}
    }
    pub fn classify(&mut self, chr: char) -> Chartype {
        if self.escaped {
            self.escaped = false;
            return Chartype::Literal;
        }
        else if chr == '\\' {
            self.escaped = true;
            return Chartype::Special;
        }
        else {
            return Chartype::get_type(chr)
        }
    }
}

struct LabeledChar {

}

#[derive(Debug)]
pub enum Chartype {
    Literal,
    Special,
    Grouping
}
impl Chartype {
    fn get_type(chr: char) -> Chartype {
        match chr {
            '.' | '+' | '?' | '{' | '}' => Chartype::Special, 
            '[' | ']' | '(' | ')' | '|' => Chartype::Grouping,
            _ => Chartype::Literal
        }
    }
}