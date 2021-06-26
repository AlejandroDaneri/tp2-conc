extern crate reqwest;

mod counter;
mod logger;
mod synonym;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::synonym::searcher::Searcher;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let log = Arc::new(logger::Logger::new(logger::Level::Debug));
    log.debug("Configure log".to_string());

    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        log.error("Missing path arg".to_string());
        return Ok(());
    }

    let path = args[1].as_str();

    log.debug("Opening file".to_string());
    let f = File::open(path)?;

    let buffered = BufReader::new(f);

    log.debug("Reading file".to_string());

    let words: Vec<String> = buffered.lines().flatten().collect();

    let earcher = Searcher::new(words);

    earcher.searchs();
    log.debug("Finish".to_string());

    Ok(())
}
