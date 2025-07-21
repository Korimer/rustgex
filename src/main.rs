mod tokenizing;
mod utils;

use std::process::exit;

use tokenizing::Pattern;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Error: should be called in the following format: \n\trustgex.exe <pattern> <to_match>");
        exit(1);
    }
    // println!("DEBUG: executed with args:");
    // let mut i = 0;
    // for arg in (&args).into_iter().skip(1) {
    //     println!("\t{i}: {arg}");
    //     i += 1;
    // }
    // println!("DEBUG: init success.");
    let mypattern = Pattern::new(&args[1]);
    // println!("DEBUG: Pattern built.");
    let trvth = mypattern.quickmatch(&args[2]);
    // println!("DEBUG: Match concluded.");
    println!("the truth is... {trvth}.",);
    println!("in other words, they {} match.",if trvth {"DO"} else {"DON'T"});
}