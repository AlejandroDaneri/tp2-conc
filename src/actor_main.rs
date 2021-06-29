mod actors;
mod counter;
mod logger;
pub mod synonym;

use actix::prelude::*;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use actors::merriamwebster::MerriamWebsterActor;
use actors::messages::WordMessage;
use actors::synonyms::SynonymsActor;
use actors::thesaurus::ThesaurusActor;
use actors::yourdictionary::YourDictionaryActor;


#[actix::main]
async fn main() {
    let log = logger::Logger::new(logger::Level::Debug);

    log.info("Search starting with actors...".to_string());

    // start new actor
    let mut synonyms_actor = SynonymsActor::new();
    let merriam_addr = SyncArbiter::start(1, MerriamWebsterActor::new);
    let your_dict_addr = SyncArbiter::start(1, YourDictionaryActor::new);
    let thes_addr = SyncArbiter::start(1, ThesaurusActor::new);

    synonyms_actor.add_dictionary_actor(thes_addr.recipient());
    synonyms_actor.add_dictionary_actor(your_dict_addr.recipient());
    synonyms_actor.add_dictionary_actor(merriam_addr.recipient());
    let addr = synonyms_actor.start();

    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Missing path arg");
        log.error("Missing path arg".to_string());
        // stop system and exit
        System::current().stop();
    }

    let path = args[1].as_str();

    log.debug("Opening file".to_string());
    let f = match File::open(path) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let buffered = BufReader::new(f);

    log.debug("Reading file".to_string());
    let mut promises = Vec::new();
    for line in buffered.lines() {
        match line {
            Ok(word) => {
                log.debug(format!("Searching synonyms for {}", word));

                // send message and get future for result
                let message = WordMessage {
                    word: word.to_owned(),
                };
                promises.push(addr.send(message))
            }
            Err(err) => log.error(format!("{:?}", err)),
        };
    }

    for promise in promises {
        match promise.await {
            Ok(Ok(counter)) => println!("RESULT: {:?}", counter.print_counter()),
            Err(err) => log.error(format!("Mailbox Promise Error: {:?}", err)),
            Ok(Err(err)) => log.error(format!("{:?}", err)),
        };
    }
    // stop system and exit
    System::current().stop();
}
