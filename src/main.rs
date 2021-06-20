extern crate reqwest;

mod counter;
mod logger;
mod synonym;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::counter::Counter;
use crate::synonym::merriamwebster::MerriamWebster;
use crate::synonym::thesaurus::Thesaurus;
use crate::synonym::yourdictionary::YourDictionary;
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
    let f = File::open(path)?;

    let buffered = BufReader::new(f);

    log.debug("Reading file".to_string());

    let words: Vec<String> = buffered.lines().flatten().collect();
    let thesaurus_queries = words.iter().map(|word| Thesaurus::new_query(&word));

    let results = thesaurus_queries.map(|query| query.find_synonyms());

    let mut counter = Counter::new();

    results
        .map(|result| {
            if result.is_err() {
                log.warn(format!(
                    "Problem getting synonyms from Thesaurus: {:?}",
                    result
                ));
            }
            result
        })
        .flatten()
        .for_each(|syn_list| {
            counter.count(&syn_list);
        });

    log.info(format!("COUNT : {:?}", counter.get_counter()));

    log.debug("Finish".to_string());

    Ok(())
}
