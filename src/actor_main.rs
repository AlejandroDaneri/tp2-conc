mod actors;
mod counter;
mod synonym;

use actix::prelude::*;

use actors::merriamwebster::MerriamWebsterActor;
use actors::messages::{DictMessage, WordMessage};
use actors::synonyms::SynonymsActor;
use actors::thesaurus::ThesaurusActor;
use actors::yourdictionary::YourDictionaryActor;
use counter::Counter;

#[actix::main]
async fn main() {
    // start new actor
    let mut synonyms_actor = SynonymsActor::new();
    let merriam_addr = SyncArbiter::start(1, move || MerriamWebsterActor::new());
    let your_dict_addr = SyncArbiter::start(1, move || YourDictionaryActor::new());
    let thes_addr = SyncArbiter::start(1, move || ThesaurusActor::new());

    synonyms_actor.add_dictionary_actor(thes_addr.recipient());
    synonyms_actor.add_dictionary_actor(your_dict_addr.recipient());
    synonyms_actor.add_dictionary_actor(merriam_addr.recipient());
    let addr = synonyms_actor.start();

    // send message and get future for result
    let message = WordMessage {
        word: "work".to_owned(),
    };
    let res = addr.send(message).await;

    println!("RESULT: {:?}", res.unwrap());

    // stop system and exit
    System::current().stop();
}
