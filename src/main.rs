use matchable::Matchable;

mod tokenize;
mod matchable;
mod match_mechanisms;
mod token_builder;
mod classifychar;

fn main() {
    let a: match_mechanisms::literal::LiteralMatcher = match_mechanisms::literal::LiteralMatcher::new(vec!['a','b','c']);
    println!("{:?}",a.matchesof("ababa"));
}