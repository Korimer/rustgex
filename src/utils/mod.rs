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
    
    pub enum ParsedChar {
        Char(char),
        Alias(Alias)
    }

    pub enum Alias {
        Character(Character),
        CharacterClass(CharacterClass)
    }

    pub enum Character {
        NewLine,
        Tab,
    }

    pub enum CharacterClass {
        Negation(Box<CharacterClass>),
        DecimalDigit,
        WordChar,
        WhiteSpace,
    }

    impl ParsedChar {
        pub fn unwrap_char(&self) -> &char {
            if let ParsedChar::Char(chr) = self {chr}
            else {panic!("Tried to unwrap a non-char")}
        }
    }
    
    impl Character {
        pub fn translate(&self) -> char {
            match *self {
                Character::NewLine => '\n',
                Character::Tab => ' ',
            }
        }
    }

    impl CharacterClass {
        pub fn is_negated(&self) -> bool {
            if let CharacterClass::Negation(_) = self {true} else {false}
        }
        pub fn negate(self) -> Self {
            match self {
                CharacterClass::Negation(chrcls) => *chrcls,
                non_negated => CharacterClass::Negation(Box::new(non_negated)),
            }
        }
    }
}