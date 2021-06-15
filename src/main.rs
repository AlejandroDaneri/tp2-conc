extern crate reqwest;

mod synonym;

use std::env::args;

use crate::synonym::Finder;
use crate::synonym::thesaurus::Thesaurus;

const ARGS_MIN_LEN: usize = 2;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let argv: Vec<String> = args().collect();
    if args().len() < ARGS_MIN_LEN {
        // println!("Usage: {:?} <input file>", argv[0]);
        println!("Usage: specify word to search as an argument");
    } else {
        let word = argv[1].clone();
        let q = Thesaurus::new_query(word.as_str());
        let synonyms = q.find_synonyms()?;
        println!("Sinonimos: {:?}", synonyms);
    }
    Ok(())
}
