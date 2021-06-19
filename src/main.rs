extern crate reqwest;

mod counter;
mod logger;
mod synonym;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::counter::count;
use crate::synonym::thesaurus::Thesaurus;
use crate::synonym::Finder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let log = logger::Logger::new(logger::Level::Debug);
    log.debug("Configure log".to_string());

    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Missing path arg");
        log.error("Missing path arg".to_string());
        return Ok(());
    }

    let path = args[1].as_str();

    log.debug("Opening file".to_string());
    let f = match File::open(path) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let buffered = BufReader::new(f);

    log.debug("Reading file".to_string());
    for line in buffered.lines() {
        let word = line.unwrap();
        log.debug(format!("Searching synonyms for {}", word));

        let synonyms_thesaurus = match Thesaurus::new_query(&word).find_synonyms() {
            Ok(syns) => syns,
            Err(error) => {
                log.warn(format!(
                    "Problem getting synonyms from Thesaurus: {:?}",
                    error
                ));
                continue;
            }
        };

        log.info(format!("Sinonimos: {:?}", synonyms_thesaurus));

        let res = count(&synonyms_thesaurus);
        log.info(format!("COUNT : {:?}", res))
    }

    log.debug("Finish".to_string());

    Ok(())
}
