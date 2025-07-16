pub mod charid {
    use std::sync::LazyLock;
    use std::collections::{HashMap,HashSet};
    static SPECIAL_CHARS: LazyLock<HashSet<char>> = LazyLock::new(|| HashSet::from([
            '\\', '^', '$', '.', '|', '?', '*', '+', '(', ')', '[', ']', '{', '}', 
    ]));
    static BRACKETS: LazyLock<HashMap<char,char>> = LazyLock::new(|| HashMap::from([
        ('(',')'),
        ('[',']'),
        ('{','}'),
    ]));

    pub fn is_special(chr: &char) -> bool {
        (*SPECIAL_CHARS).contains(chr)
    }
    pub fn match_bracket(chr: &char) -> Option<char> {
        (*BRACKETS).get(chr).copied()
    }

    pub enum CharAlias {
        OpenCurly,
        ClosedCurly
    }
    impl CharAlias {
        pub fn identify(&self) -> char {
            match self {
                Self::OpenCurly => '{',
                Self::ClosedCurly => '}',
            }
        }
    }
}

pub mod regex_aliases {
    
    //Restructuring: 
    //  parsedchar: character | charset
    //  character: char | charalias
    //Reason: Simplifies the decision tree to an absurd degree  

    #[derive(Debug)]
    pub enum ParsedChar {
        Char(char),
        Alias(Alias)
    }
    #[derive(Debug)]
    pub enum Alias {
        Character(Character),
        CharacterClass(CharacterClass)
    }
    #[derive(Debug)]
    pub enum Character {
        NewLine,
        Tab,
    }
    #[derive(Debug)]
    pub enum CharacterClass {
        Negation(Box<CharacterClass>),
        DecimalDigit,
        WordChar,
        WhiteSpace,
    }

    impl ParsedChar {
        pub fn contains_set(&self) -> bool {
            matches!(self,ParsedChar::Alias(Alias::CharacterClass(_)))
        }
        pub fn contains_char(&self) -> bool{!self.contains_set()}

        pub fn unwrap_char(&self) -> char {
            if let Self::Char(chr) = self {*chr} // clones...
            else if let Self::Alias(Alias::Character(chr)) = self {chr.translate()}
            else {panic!("Tried to unwrap a non-char")}
        }
    }

    impl Alias {
        pub fn escaped_to_alias(escaped_char: char) -> Option<Alias> {
            let options = &[
                Character::escaped_to_character,
                CharacterClass::escaped_to_class,
            ];
            for opt in options {
                let result = opt(escaped_char);
                if result.is_some() {return result}
            }
            None
        }
    }

    impl Character {
        pub fn escaped_to_character(escaped_char: char) -> Option<Alias> {
            println!("DEBUG: mapping \\{escaped_char} to alias");
            match escaped_char {
                'n' => Some(Self::NewLine),
                't' => Some(Self::Tab),
                _ => None
            }
            .map(|character| Alias::Character(character))
        }

        pub fn translate(&self) -> char {
            match *self {
                Self::NewLine => '\n',
                Self::Tab => '\t',
            }
        }
    }

    impl CharacterClass {
        pub fn escaped_to_class(escaped_char: char) -> Option<Alias> {
            let negated = escaped_char.is_uppercase(); // might break on non utf8, investigate
            let lower_char = if negated {escaped_char.to_ascii_lowercase()} else {escaped_char};
            match lower_char {
                'w' => Some(Self::WordChar),
                _ => None
            }
            .map(|chrclass| if negated {Self::Negation(Box::new(chrclass))} else {chrclass})
            .map(|chrclass| Alias::CharacterClass(chrclass))
        }
        pub fn is_negated(&self) -> bool {
            if let Self::Negation(_) = self {true} else {false}
        }
        pub fn negate(self) -> Self {
            match self {
                Self::Negation(chrcls) => *chrcls,
                non_negated => Self::Negation(Box::new(non_negated)),
            }
        }
    }
}

pub mod regex_reader {
    use crate::utils::regex_aliases::{Alias, ParsedChar};

    pub struct RegExReader<'a> {
        escaped: bool,
        patterntext: &'a str,
    }

    impl <'a> RegExReader<'a> {
        fn init(pattern: &'a str) -> Self {
            Self { 
                escaped: false,
                patterntext: pattern, 
            }
        }

        pub fn new(pattern: &'a str) -> Vec<ParsedChar> {
            let mut reader = Self::init(pattern);
            let mut txtiter = pattern.chars().into_iter();
            let mut fullptrn = Vec::new();
            while let Some(chr) = txtiter.next() {
                if let Some(parsed) = reader.read(chr) {
                    fullptrn.push(parsed);
                }
            }
            fullptrn
        }

        fn read(&mut self, chr: char) -> Option<ParsedChar> {
            if self.escaped {
                self.escaped = false;
                if let Some(alias) = Alias::escaped_to_alias(chr) {
                    Some(ParsedChar::Alias(alias))
                } else {
                    Some(ParsedChar::Char(chr))
                }
            }
            else {
                if chr == '\\' {
                    self.escaped = true;
                    None
                } else {
                    Some(ParsedChar::Char(chr))
                }
            }
        }
    }
}