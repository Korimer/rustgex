mod literal;
mod set;
mod any;

use crate::utils::regex_aliases::*;

use literal::LiteralMatcher;
use set::SetMatcher;
use any::AnyMatcher;
use super::multiple::MultipleMatcher;
use super::matching::{Matchable, Extensible, TokenMorph, TryMorph};

//technically this should be matcher-maker... but matchmaker is funnier
type MatchMaker = fn(char) -> Option<Box<dyn GeneralIndividualMatcher>>;

const PRECEDENCE: &[MatchMaker] = &[
    AnyMatcher::try_create,
    SetMatcher::try_create,
    LiteralMatcher::try_create,
];

pub struct IndividualMatcher(Box<dyn GeneralIndividualMatcher>);

impl IndividualMatcher {
    pub fn from (tkn: ParsedChar) -> IndividualMatcher {
        match tkn {
            ParsedChar::Char(chr) => Self::from_chr(chr),
            ParsedChar::Alias(alias) => Self::from_alias(alias),
        }
    }

    fn from_chr(chr: char)-> IndividualMatcher {
        for matchertype in PRECEDENCE.iter() {
            if let Some(matcher) = matchertype(chr) {
                return IndividualMatcher(matcher);
            }
        }
        unreachable!()
    }

    fn from_alias(als: Alias) -> IndividualMatcher {
        match als {
            Alias::Character(chr) => Self::from_chr(chr.translate()),
            Alias::CharacterClass(chrcls) => Self::from_set_alias(chrcls),
        }
    }

    fn from_set_alias(cls: CharacterClass) -> IndividualMatcher {
        let negated = cls.is_negated(); 
        let true_cls = if negated {cls.negate()} else {cls};

        let preset: &[char] = match true_cls {
            CharacterClass::Negation(_) => unreachable!(),
            CharacterClass::DecimalDigit => &('0'..='9').collect::<Vec<_>>(),
            CharacterClass::WordChar => &('a'..='z').collect::<Vec<_>>(),
            CharacterClass::WhiteSpace => &[' ', '\t',' '],
        };

        let mut fullset = SetMatcher::from(preset);
        if negated {fullset.negate()}
        return IndividualMatcher(Box::new(fullset))
    }
}

impl Matchable for IndividualMatcher {
    fn matches(&self, tomatch: &Vec<char>, startind: usize) -> Vec<usize> {
        (*self.0).matches(tomatch,startind)
    }
}

impl Extensible for IndividualMatcher {
    fn extend(&mut self, chr: char) -> bool {
        self.0.extend(chr)
    }
}

impl TokenMorph for IndividualMatcher {
    fn gettarget(&self) -> TryMorph {
        TryMorph::new::<MultipleMatcher>()
    }
    fn dyn_self(self: Box<Self>) -> Box<dyn TokenMorph> {
        self
    }
}

pub trait GeneralIndividualMatcher: Extensible {
    fn try_create(chr: char) -> Option<Box<dyn GeneralIndividualMatcher>> where Self: Sized;
}