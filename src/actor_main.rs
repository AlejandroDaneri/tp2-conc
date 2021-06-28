mod actors;
mod counter;
mod logger;
mod synonym;

use actix::prelude::*;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use actors::merriamwebster::MerriamWebsterActor;
use actors::messages::{DictMessage, WordMessage};
use actors::synonyms::SynonymsActor;
use actors::thesaurus::ThesaurusActor;
use actors::yourdictionary::YourDictionaryActor;
use counter::Counter;

use crate::actors::messages::AddActor;

#[actix::main]
async fn main() {
    let log = logger::Logger::new(logger::Level::Debug);

    let merriam_addr = MerriamWebsterActor::new().start();
    let your_dict_addr = YourDictionaryActor::new().start();
    let thes_addr = ThesaurusActor::new().start();

    let synonyms_actor = SyncArbiter::start(2, SynonymsActor::new);

    synonyms_actor.send(AddActor {
        addr: merriam_addr.recipient(),
    });

    synonyms_actor.send(AddActor {
        addr: your_dict_addr.recipient(),
    });

    synonyms_actor.send(AddActor {
        addr: thes_addr.recipient(),
    });

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
        // promises.push(synonyms_actor.send(message))
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
