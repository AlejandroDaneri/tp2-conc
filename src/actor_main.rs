mod actors;
mod counter;
mod synonym;

use actix::prelude::*;

use actors::messages::{DictMessage, WordMessage};
use actors::synonyms::SynonymsActor;
use actors::thesaurus::ThesaurusActor;
use actors::yourdictionary::YourDictionaryActor;
use counter::Counter;

#[actix::main]
async fn main() {
    // start new actor
    let mut synonyms_actor = SynonymsActor::new();
    let thesaurus = ThesaurusActor::new();
    let thes_addr = thesaurus.start();
    let your_dict = YourDictionaryActor::new();
    let your_dict_addr = your_dict.start();
    synonyms_actor.add_dictionary_actor(thes_addr.recipient());
    synonyms_actor.add_dictionary_actor(your_dict_addr.recipient());
    let addr = synonyms_actor.start();

    // send message and get future for result
    let message = WordMessage {
        word: "capybara".to_owned(),
    };
    let res = addr.send(message).await;

    println!("RESULT: {:?}", res.unwrap());

    // stop system and exit
    System::current().stop();
}
