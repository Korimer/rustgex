use super::matchable::Matchable;

pub struct Group {
    tokens: Vec<Box<dyn Matchable>>
}