use matchable::Matchable;

mod tokenize;
mod matchable;
fn main() {
    let a = tokenize::Literal::new("aba");
    println!("{:?}",a.matchesof("ababa"))
}