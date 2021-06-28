use std::time::Duration;

use crate::{
    synonym::{thesaurus::Thesaurus, Finder, FinderError},
    DictMessage,
};
use actix::{
    prelude::{Actor, Handler},
    AsyncContext, Context, WrapFuture,
};

pub struct ThesaurusActor {}

impl ThesaurusActor {
    pub fn new() -> Self {
        Self {}
    }
}

/// Declare actor and its context
impl Actor for ThesaurusActor {
    type Context = Context<Self>;
}

/// Handler for `WordMessage` message
impl Handler<DictMessage> for ThesaurusActor {
    type Result = Result<Vec<String>, Box<dyn std::error::Error + Send>>;

    fn handle(&mut self, msg: DictMessage, ctx: &mut Context<Self>) -> Self::Result {
        let time_to_wait = Duration::from_millis(1000);
        ctx.wait(actix::clock::sleep(time_to_wait).into_actor(self));
        match Thesaurus::new_query(&msg.word).find_synonyms() {
            Ok(res) => Ok(res),
            Err(_) => Err(Box::new(FinderError {})),
        }
    }
}
