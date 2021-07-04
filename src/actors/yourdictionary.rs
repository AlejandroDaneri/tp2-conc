//! Modulo encargado de la busqueda sobre la pagina https://thesaurus.yourdictionary.com/

use crate::{
    actors::messages::DictMessage,
    synonym::{yourdictionary::YourDictionary, Finder, FinderError, QueryResponse},
};
use actix::{
    prelude::{Actor, Handler},
    AsyncContext, Context, WrapFuture,
};

use std::time::{Duration, SystemTime};

/// Actor encargado de la busqueda sobre la pagina https://thesaurus.yourdictionary.com/
pub struct YourDictionaryActor {
    last_search_time: SystemTime,
}

impl YourDictionaryActor {
    /// Genera un YourDictionaryActor
    pub fn new() -> Self {
        let last_search_time = SystemTime::UNIX_EPOCH; //TODO: poner en un actor nuevo
        Self { last_search_time }
    }

    pub fn sleep_if_necessary(&mut self, page_cooldown: u64, ctx: &mut Context<Self>) {
        let now = SystemTime::now();
        let duration = match now.duration_since(self.last_search_time) {
            Ok(duration) => duration,
            _ => unreachable!(),
        };
        if duration.as_secs() < page_cooldown {
            ctx.wait(
                actix::clock::sleep(Duration::from_secs(page_cooldown - duration.as_secs()))
                    .into_actor(self),
            );
        }
        self.last_search_time = now;
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
    type Result = Result<QueryResponse, Box<dyn std::error::Error + Send>>;

    fn handle(&mut self, msg: DictMessage, ctx: &mut Context<Self>) -> Self::Result {
        self.sleep_if_necessary(msg.page_cooldown, ctx);
        if let Ok(res) = YourDictionary::new_query(&msg.word).find_synonyms() {
            Ok(res)
        } else {
            Err(Box::new(FinderError {}))
        }
    }
}
