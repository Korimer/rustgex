mod tokenizing;
mod utils;

use tokenizing::Pattern;
fn main() {
    println!("well we good thus far");
    let mypattern = Pattern::new("a");
    println!("well we built");
    println!("the truth is... {}",mypattern.quickmatch("a"));
}