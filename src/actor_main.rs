mod actors;
mod counter;
mod logger;
mod requester;
pub mod synonym;

use actors::requester::RequesterActor;

use actix::prelude::*;

use actors::thesaurus::ThesaurusActor;

use std::env;

use std::fs::File;
use std::io::{BufRead, BufReader};

use actors::messages::WordMessage;
use actors::synonyms::SynonymsActor;

use actors::yourdictionary::YourDictionaryActor;

#[actix::main]
async fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();
    let log = logger::Logger::new(logger::Level::Debug);

    if args.len() <= 3 {
        println!("Malformated arguments. Expected: [FILE_PATH] [COOLDOWN] [MAX_CONCURRENTE_REQS]");
        log.error("Missing arguments".to_string());
        return Ok(());
    }

    let max_conc_reqs = match args[3].parse::<usize>() {
        Ok(num) => num,
        Err(err) => {
            log.error(format!("Error when parsing max_conc_reqs {}", err));
            return Err(());
        }
    };

    let page_cooldown = match args[2].parse::<u64>() {
        Ok(num) => num,
        Err(err) => {
            log.error(format!("Error when parsing page_cooldown {}", err));
            return Err(());
        }
    };

    let path = args[1].as_str();

    run_search(log, path, page_cooldown, max_conc_reqs).await
}

async fn run_search(
    log: logger::Logger,
    path: &str,
    page_cooldown: u64,
    max_conc_reqs: usize,
) -> Result<(), ()> {
    log.info("Search starting with actors...".to_string());
    let requester_addr = SyncArbiter::start(max_conc_reqs, RequesterActor::new);
    // let merriam_addr = MerriamWebsterActor::new(requester_addr.clone()).start();
    let your_dict_addr = YourDictionaryActor::new(requester_addr.clone()).start();
    let thes_addr = ThesaurusActor::new(requester_addr.clone()).start();

    let mut synonyms_actor = SynonymsActor::new();

    synonyms_actor.add_dictionary_actor(your_dict_addr.recipient());
    synonyms_actor.add_dictionary_actor(thes_addr.recipient());

    log.debug("Opening file".to_string());
    let f = match File::open(path) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let buffered = BufReader::new(f);

    log.debug("Reading file".to_string());
    let mut words = Vec::new();
    for line in buffered.lines() {
        match line {
            Ok(word) => {
                words.push(word);
            }
            Err(err) => log.error(format!("{:?}", err)),
        };
    }
    // send message and get future for result
    let message = WordMessage {
        word: words,
        page_cooldown,
    };

    match synonyms_actor.start().send(message).await {
        Ok(Ok(counter)) => println!("{:?}", counter),
        Err(err) => log.error(format!("Mailbox Promise Error: {:?}", err)),
        Ok(Err(err)) => log.error(format!("{:?}", err)),
    };

    log.info("Finish".to_string());
    // stop system and exit
    System::current().stop();
    Ok(())
}
