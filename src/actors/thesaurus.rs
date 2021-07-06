//! Modulo encargado de la busqueda sobre la pagina https://www.thesaurus.com/browse/

use crate::{
    actors::messages::DictMessage,
    synonym::{thesaurus::Thesaurus, Finder, FinderError, QueryResponse},
};
use actix::{
    prelude::{Actor, Handler},
    SyncContext,
};
use std::thread;
use std::time::{Duration, SystemTime};

/// Actor encargado de la busqueda sobre la pagina https://www.thesaurus.com/browse/

pub struct ThesaurusActor {
    last_search_time: SystemTime,
}

impl ThesaurusActor {
    /// Genera un ThesaurusActor
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

impl Default for ThesaurusActor {
    fn default() -> Self {
        ThesaurusActor::new()
    }
}

/// Declare actor and its context
impl Actor for ThesaurusActor {
    type Context = SyncContext<Self>;
}

/// Handler for `WordMessage` message
impl Handler<DictMessage> for ThesaurusActor {
    type Result = Result<QueryResponse, Box<dyn std::error::Error + Send>>;

    fn handle(&mut self, msg: DictMessage, _: &mut SyncContext<Self>) -> Self::Result {
        self.sleep_if_necessary(msg.page_cooldown);
        if let Ok(res) = Thesaurus::new_query(&msg.word).find_synonyms() {
            Ok(res)
        } else {
            Err(Box::new(FinderError {}))
        }
    }
}
