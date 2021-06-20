extern crate reqwest;

mod counter;
mod logger;
mod synonym;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::slice::Iter;
use std::thread::JoinHandle;

use std::thread;

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
    let thesaurus_queries = words.iter().map(create_query::<Thesaurus>);
    let merriam_queries = words.iter().map(create_query::<MerriamWebster>);
    let your_dict_queries = words.iter().map(create_query::<YourDictionary>);

    let all_queries = thesaurus_queries.chain(merriam_queries).chain(your_dict_queries);

    let handles: Vec<JoinHandle<_>> = all_queries
        .map(|query| thread::spawn(move || query.find_synonyms()))
        .collect();

    let results = handles.into_iter().map(|handle| handle.join());

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
        .flatten()
        .for_each(|syn_list| {
            counter.count(&syn_list);
        });

    log.info(format!("COUNT : {:?}", counter.get_counter()));

    log.debug("Finish".to_string());

    Ok(())
}

fn create_query<T: 'static + Finder + Send>(word: &String) -> Box<dyn Finder + Send> {
    Box::new(T::new_query(&word))
}
