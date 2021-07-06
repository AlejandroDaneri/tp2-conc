mod actors;
mod counter;
mod logger;
mod requester;
pub mod synonym;

use crate::actors::messages::RequestMessage;
use crate::actors::requester::RequesterActor;
use crate::synonym::thesaurus::Thesaurus;
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
    let merriam_addr = MerriamWebsterActor::new();
    let your_dict_addr = YourDictionaryActor::new();
    let thes_addr = ThesaurusActor::new();

    thes_addr.add_requester(requester_addr);
    your_dict_addr.add_requester(requester_addr);
    merriam_addr.add_requester(requester_addr);

    merriam_addr.start();
    your_dict_addr.start();
    thes_addr.start();

    // start new actor
    let mut synonyms_actor = SynonymsActor::new();

    synonyms_actor.add_dictionary_actor(thes_addr.recipient());
    synonyms_actor.add_dictionary_actor(your_dict_addr.recipient());
    synonyms_actor.add_dictionary_actor(merriam_addr.recipient());
    let addr = synonyms_actor.start();

    let request_addr = SyncArbiter::start(1, RequesterActor::new);
    let message = RequestMessage::<Thesaurus>::new("Hello");
    let response = request_addr.send(message).await;
    println!("Response: {:?}", response);

    log.debug("Opening file".to_string());
    let f = match File::open(path) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let buffered = BufReader::new(f);

    log.debug("Reading file".to_string());
    let mut promises = Vec::new();
    let words = Vec::new();
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
    promises.push(addr.send(message));
    for promise in promises {
        match promise.await {
            Ok(Ok(counter)) => println!("{}", counter),
            Err(err) => log.error(format!("Mailbox Promise Error: {:?}", err)),
            Ok(Err(err)) => log.error(format!("{:?}", err)),
        };
    }
    log.info("Finish".to_string());
    // stop system and exit
    System::current().stop();
    Ok(())
}
