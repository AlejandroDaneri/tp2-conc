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
use crate::synonym::searcher::search_word;

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
    let sem = Arc::new(Semaphore::new(2));

    let mut word_handles = Vec::new();
    for word in words {
        let c_sem = sem.clone();
        let c_log = log.clone();
        word_handles.push(thread::spawn(move || {
            let mut handles = Vec::new();

            search_word(c_sem.clone(), word, &mut handles);

            let results = handles.into_iter().map(|handle| handle.join());
            let mut counter = Counter::new();

            results
                .map(|result| {
                    if result.is_err() {
                        c_log.warn(format!("Problem getting synonyms: {:?}", result));
                    }
                    result
                })
                .flatten()
                .flatten()
                .for_each(|syn_list| {
                    counter.count(&syn_list);
                });

            c_log.info(format!("COUNT : {:?}", counter.get_counter()));
        }));
    }
    for thread in word_handles {
        thread
            .join()
            .map_err(|err| println!("Word join error {:?}", err))
            .ok();
    }
    log.debug("Finish".to_string());

    Ok(())
}
