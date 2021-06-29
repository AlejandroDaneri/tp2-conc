//! Modulo encargado de la busqueda sobre la pagina https://www.merriam-webster.com/thesaurus/

use crate::{
    actors::messages::DictMessage,
    synonym::{merriamwebster::MerriamWebster, Finder, FinderError},
};
use actix::prelude::{Actor, Handler};
use actix::SyncContext;

/// Actor encargado de la busqueda sobre la pagina https://www.merriam-webster.com/thesaurus/
pub struct MerriamWebsterActor {}

impl MerriamWebsterActor {
    /// Genera un MerriamWebsterActor
    pub fn new() -> Self {
        Self {}
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
        if let Ok(res) = MerriamWebster::new_query(&msg.word).find_synonyms() {
            Ok(res)
        } else {
            Err(Box::new(FinderError {}))
        }
    }
}
