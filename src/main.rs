extern crate reqwest;

mod synonym;
mod logger;

use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

use crate::synonym::thesaurus::Thesaurus;
use crate::synonym::Finder;


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let log = logger::Logger::new(logger::Level::Debug);
    log.debug(format!("Configure log"));

    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Missing path arg");
        log.error(format!("Missing path arg"));
        return Ok(())
    }

    let path= args[1].as_str();

    log.debug(format!("Opening file"));
    let f = match File::open(path) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let buffered = BufReader::new(f);

    log.debug(format!("Reading file"));
    for line in buffered.lines() {
        let word = line.unwrap();
        log.debug(format!("Searching synonyms for {}", word));

        let synonyms_thesaurus = match Thesaurus::new_query(&word).find_synonyms() {
            Ok(syns) => syns,
            Err(error) => {
                log.warn(format!("Problem getting synonyms from Thesaurus: {:?}", error));
                continue
            },
        };

        log.info(format!("Sinonimos: {:?}", synonyms_thesaurus));

    }

    log.debug(format!("Finish"));

    Ok(())

}
