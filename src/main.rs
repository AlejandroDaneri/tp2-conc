extern crate reqwest;

use reqwest::blocking;
use std::env::args;

const ARGS_MIN_LEN: usize = 2;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let argv : Vec<String> = args().collect();
    if args().len() < ARGS_MIN_LEN {
        println!("Usage: {:?} <input file>", argv[0]);
    } else {
        let request = blocking::get("https://thesaurus.yourdictionary.com/test")?;
        println!("request -> {:?}", request);
        println!("request.text() -> {:?}", request.text()?);
    }
    Ok(())
}