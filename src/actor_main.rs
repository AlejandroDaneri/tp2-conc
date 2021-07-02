mod actors;
mod counter;
mod logger;
pub mod synonym;

use actix::prelude::*;

use std::env;

use std::fs::File;
use std::io::{BufRead, BufReader};

use actors::merriamwebster::MerriamWebsterActor;
use actors::messages::AddrMessage;
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

    let synonyms_actor = SyncArbiter::start(max_conc_reqs, SynonymsActor::new);
    let merriam_addr = MerriamWebsterActor::new().start().recipient();
    let your_dict_addr = YourDictionaryActor::new().start().recipient();
    let thes_addr = ThesaurusActor::new().start().recipient();

    synonyms_actor
        .send(AddrMessage { addr: merriam_addr })
        .await
        .unwrap();
    synonyms_actor
        .send(AddrMessage {
            addr: your_dict_addr,
        })
        .await
        .unwrap();
    synonyms_actor
        .send(AddrMessage { addr: thes_addr })
        .await
        .unwrap();
    // let addr = synonyms_actor.start();

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
                    page_cooldown,
                };
                promises.push(synonyms_actor.send(message))
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
    log.info("Finish".to_string());
    // stop system and exit
    System::current().stop();
    Ok(())
}
