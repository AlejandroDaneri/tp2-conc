extern crate reqwest;

mod counter;
mod logger;
mod synonym;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::u64;

use crate::synonym::searcher::Searcher;
use std::sync::Arc;

use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct ArgError<'a>(&'a str);

impl<'a> Error for ArgError<'a> {}

impl<'a> fmt::Display for ArgError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let log = Arc::new(logger::Logger::new(logger::Level::Debug));
    log.info("Search starting...".to_string());

    let args: Vec<String> = env::args().collect();

    if args.len() <= 3 {
        println!("Malformated arguments. Expected: [FILE_PATH] [COOLDOWN] [MAX_CONCURRENTE_REQS]");
        log.error("Missing arguments".to_string());
        return Ok(());
    }

    let path = args[1].as_str();

    let max_conc_reqs = match args[3].parse::<usize>() {
        Ok(num) => num,
        Err(err) => {
            log.error(format!("Error when parsing max_conc_reqs {}", err));
            return Err(Box::new(ArgError("Error parsing max conc reqs")));
        }
    };

    let page_cooldown = match args[2].parse::<u64>() {
        Ok(num) => num,
        Err(err) => {
            log.error(format!("Error when parsing page_cooldown {}", err));
            return Err(Box::new(ArgError("Error parsing page cooldown")));
        }
    };

    log.debug("Opening file".to_string());
    let f = File::open(path)?;

    let buffered = BufReader::new(f);

    log.debug("Reading file".to_string());

    let words: Vec<String> = buffered.lines().flatten().collect();

    let searcher = Searcher::new(words);

    searcher.searchs(page_cooldown, max_conc_reqs);
    log.info("Finish".to_string());

    Ok(())
}
