use matchable::Matchable;

mod tokenize;
mod matchable;
mod match_mechanisms;
mod token_builder;
mod classifychar;

fn main() {
    let a = match_mechanisms::literal::LiteralMatcher::new("aba");
    println!("{:?}",a.matchesof("ababa"));
    let mut b = classifychar::Classifier::new();
    println!("{:?}",b.classify('\\'));
}