//! Modulo encargado de la busqueda sobre la pagina https://thesaurus.yourdictionary.com/

use crate::{
    actors::messages::DictMessage,
    synonym::{thesaurus::Thesaurus, Finder},
};
use actix::{
    prelude::{Actor, Handler},
    Addr, AsyncContext, Context, WrapFuture,
};
use std::thread;
use std::time::{Duration, SystemTime};

/// Actor encargado de la busqueda sobre la pagina https://thesaurus.yourdictionary.com/
pub struct YourDictionaryActor {
    // last_search_time: SystemTime,
    requester: Addr,
}

impl YourDictionaryActor {
    /// Genera un YourDictionaryActor
    pub fn new() -> Self {
        // let last_search_time = SystemTime::UNIX_EPOCH;
        Self { requester: None }
    }

    pub fn sleep_if_necessary(&mut self, page_cooldown: u64) {
        let now = SystemTime::now();
        let duration = match now.duration_since(self.last_search_time) {
            Ok(duration) => duration,
            _ => unreachable!(),
        };
        if duration.as_secs() < page_cooldown {
            thread::sleep(Duration::from_secs(page_cooldown - duration.as_secs()));
        }
        self.last_search_time = now;
    }

    pub fn add_requester(&mut self, requester: Addr) {
        self.requester = requester
    }
}

impl Default for YourDictionaryActor {
    fn default() -> Self {
        YourDictionaryActor::new()
    }
}

/// Declare actor and its context
impl Actor for YourDictionaryActor {
    type Context = Context<Self>;
}

/// Handler for `WordMessage` message
impl Handler<DictMessage> for YourDictionaryActor {
    type Result = Result<Vec<String>, Box<dyn std::error::Error + Send>>;

    fn handle(&mut self, msg: DictMessage, ctx: &mut Context<Self>) -> Self::Result {
        let words = msg.word;

        for word in words {
            ctx.wait(actix::clock::sleep(Duration::from_secs(msg.page_cooldown)).into_actor(self));
            let result = self
                .requester
                .send(Thesaurus::new_query(&msg.word).url())
                .wait();
            let syns = match result.find_synonyms() {
                Ok(syns) => syns,
                Err(_) => todo!(),
            };
            //
        }
    }
}
