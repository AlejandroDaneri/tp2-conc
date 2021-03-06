//! Modulo encargado de la busqueda sobre la pagina https://thesaurus.yourdictionary.com/

use crate::{
    actors::messages::DictMessage, actors::requester::RequesterActor, counter::Counter, logger,
    synonym::merriamwebster::MerriamWebster,
};
use actix::{
    prelude::{Actor, Handler},
    Addr, Context, ResponseFuture,
};

use std::time::{Duration, SystemTime};

use super::messages::RequestMessage;

/// Actor encargado de la busqueda sobre la pagina https://thesaurus.yourdictionary.com/
pub struct YourDictionaryActor {
    requester: Addr<RequesterActor>,
}

impl YourDictionaryActor {
    /// Genera un YourDictionaryActor
    pub fn new(requester: Addr<RequesterActor>) -> Self {
        Self { requester }
    }
}

/// Declare actor and its context
impl Actor for YourDictionaryActor {
    type Context = Context<Self>;
}

/// Handler for `WordMessage` message
impl Handler<DictMessage> for YourDictionaryActor {
    type Result = ResponseFuture<Result<Vec<Counter>, Box<dyn std::error::Error + Send>>>;

    fn handle(&mut self, msg: DictMessage, _ctx: &mut Context<Self>) -> Self::Result {
        let words = msg.word.clone();
        let mut counters = Vec::new();
        let requester = self.requester.clone();
        let mut last_search_time = SystemTime::UNIX_EPOCH;

        Box::pin(async move {
            let log = logger::Logger::new(logger::Level::Debug);
            for word in words {
                let now = SystemTime::now();
                let duration = match now.duration_since(last_search_time) {
                    Ok(duration) => duration,
                    _ => {
                        log.error("Error calculating duration".to_string());
                        panic!()
                    }
                };
                if duration.as_secs() < msg.page_cooldown {
                    log.debug(format!(
                        "Your Waiting {} seconds",
                        msg.page_cooldown - duration.as_secs()
                    ));
                    actix::clock::sleep(Duration::from_secs(
                        msg.page_cooldown - duration.as_secs(),
                    ))
                    .await;
                }
                last_search_time = SystemTime::now();
                let promise = requester.send(RequestMessage::<MerriamWebster>::new(&word));
                let response = promise.await;
                match response {
                    Ok(Ok(res)) => {
                        let mut counter = Counter::new(res.word);
                        counter.count(&res.synonyms);
                        counters.push(counter);
                    }
                    Ok(Err(_err)) => log.error("Error parsing QueryResponse".to_string()),
                    Err(_err) => log.error(" Mailbox Error".to_string()),
                }
            }
            Ok(counters)
        })
    }
}
