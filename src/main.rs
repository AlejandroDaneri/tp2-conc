extern crate reqwest;

mod counter;
mod logger;
mod synonym;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::sync::Arc;
use std::thread;
use std_semaphore::Semaphore;

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
    let sem = Arc::new(Semaphore::new(5));

    for word in words {
        let mut handles = Vec::new();

        let mut c_sem = sem.clone();
        let mut c_word = word.clone();
        handles.push(thread::spawn(move || {
            let _guard = c_sem.access();
            search::<Thesaurus>(&c_word)
        }));

        c_sem = sem.clone();
        c_word = word.clone();
        handles.push(thread::spawn(move || {
            let _guard = c_sem.access();
            search::<MerriamWebster>(&c_word)
        }));

        c_sem = sem.clone();
        c_word = word.clone();
        handles.push(thread::spawn(move || {
            let _guard = c_sem.access();
            search::<YourDictionary>(&c_word)
        }));
        //TODO: implementar de otra manera para que el join no trabe los requests
        let results = handles.into_iter().map(|handle| handle.join());
        let mut counter = Counter::new();

        results
            .map(|result| {
                if result.is_err() {
                    log.warn(format!("Problem getting synonyms: {:?}", result));
                }
                result
            })
            .flatten()
            .flatten()
            .for_each(|syn_list| {
                counter.count(&syn_list);
            });

        log.info(format!("COUNT : {:?}", counter.get_counter()));
    }
    log.debug("Finish".to_string());

    Ok(())
}

fn search<T: 'static + Finder + Send>(word: &str) -> Result<Vec<String>, synonym::FinderError> {
    Box::new(T::new_query(word)).find_synonyms()
}
