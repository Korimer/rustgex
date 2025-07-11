mod tokenizing;
mod utils;

use tokenizing::Pattern;
fn main() {
    println!("DEBUG: init success");
    let mypattern = Pattern::new("\\wa");
    println!("DEBUG: Pattern built");
    println!("the truth is... {}",mypattern.quickmatch("aa"));
}