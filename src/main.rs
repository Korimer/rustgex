use matchable::Matchable;

mod tokenize;
mod matchable;
mod match_mechanisms;
fn main() {
    let a = match_mechanisms::literal::Literal::new("aba");
    println!("{:?}",a.matchesof("ababa"))
}