use crate::{
    synonym::{merriamwebster::MerriamWebster, Finder, FinderError},
    DictMessage
};
use actix::prelude::{Actor, Handler};
use actix::SyncContext;

pub struct MerriamWebsterActor {}

impl MerriamWebsterActor {
    pub fn new() -> Self {
        Self {}
    }
}

/// Declare actor and its context
impl Actor for MerriamWebsterActor {
    type Context = SyncContext<Self>;
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