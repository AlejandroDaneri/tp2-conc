//! Modulo encargado de la busqueda sobre la pagina https://thesaurus.yourdictionary.com/

use crate::{
    actors::messages::DictMessage,
    synonym::{yourdictionary::YourDictionary, Finder, FinderError},
};
use actix::{
    prelude::{Actor, Handler},
    SyncContext,
};

/// Actor encargado de la busqueda sobre la pagina https://thesaurus.yourdictionary.com/
pub struct YourDictionaryActor {}

impl YourDictionaryActor {
    /// Genera un YourDictionaryActor
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for YourDictionaryActor {
    fn default() -> Self {
        YourDictionaryActor::new()
    }
}

/// Declare actor and its context
impl Actor for YourDictionaryActor {
    type Context = SyncContext<Self>;
}

/// Handler for `WordMessage` message
impl Handler<DictMessage> for YourDictionaryActor {
    type Result = Result<Vec<String>, Box<dyn std::error::Error + Send>>;

    fn handle(&mut self, msg: DictMessage, _: &mut SyncContext<Self>) -> Self::Result {
        if let Ok(res) = YourDictionary::new_query(&msg.word).find_synonyms() {
            Ok(res)
        } else {
            Err(Box::new(FinderError {}))
        }
    }
}
