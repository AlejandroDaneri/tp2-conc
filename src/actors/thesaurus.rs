//! Modulo encargado de la busqueda sobre la pagina https://www.thesaurus.com/browse/

use crate::{
    actors::messages::DictMessage,
    synonym::{thesaurus::Thesaurus, Finder, FinderError},
};
use actix::{
    prelude::{Actor, Handler},
    SyncContext,
};

/// Actor encargado de la busqueda sobre la pagina https://www.thesaurus.com/browse/

pub struct ThesaurusActor {}

impl ThesaurusActor {
    /// Genera un ThesaurusActor
    pub fn new() -> Self {
        Self {}
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
    type Result = Result<Vec<String>, Box<dyn std::error::Error + Send>>;

    fn handle(&mut self, msg: DictMessage, _: &mut SyncContext<Self>) -> Self::Result {
        if let Ok(res) = Thesaurus::new_query(&msg.word).find_synonyms() {
            Ok(res)
        } else {
            Err(Box::new(FinderError {}))
        }
    }
}
