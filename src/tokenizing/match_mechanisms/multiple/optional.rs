use crate::tokenizing::match_mechanisms;
use match_mechanisms::matching::Matchable;

pub struct IndefiniteMatcher(Box<dyn Matchable>);