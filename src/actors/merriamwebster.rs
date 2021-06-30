//! Modulo encargado de la busqueda sobre la pagina https://www.merriam-webster.com/thesaurus/
use crate::{
    actors::messages::DictMessage,
    synonym::{merriamwebster::MerriamWebster, Finder, FinderError},
};
use actix::prelude::{Actor, Handler};
use actix::SyncContext;
use std::thread;
use std::time::{Duration, SystemTime};

/// Actor encargado de la busqueda sobre la pagina https://www.merriam-webster.com/thesaurus/
pub struct MerriamWebsterActor {
    last_search_time: SystemTime,
}

impl MerriamWebsterActor {
    /// Genera un MerriamWebsterActor
    pub fn new() -> Self {
        let last_search_time = SystemTime::UNIX_EPOCH;
        Self { last_search_time }
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
}

/// Declare actor and its context
impl Actor for MerriamWebsterActor {
    type Context = SyncContext<Self>;
}

impl Default for MerriamWebsterActor {
    fn default() -> Self {
        MerriamWebsterActor::new()
    }
}

/// Handler for `WordMessage` message
impl Handler<DictMessage> for MerriamWebsterActor {
    type Result = Result<Vec<String>, Box<dyn std::error::Error + Send>>;

    fn handle(&mut self, msg: DictMessage, _: &mut SyncContext<Self>) -> Self::Result {
        self.sleep_if_necessary(msg.page_cooldown);
        if let Ok(res) = MerriamWebster::new_query(&msg.word).find_synonyms() {
            Ok(res)
        } else {
            Err(Box::new(FinderError {}))
        }
    }
}
