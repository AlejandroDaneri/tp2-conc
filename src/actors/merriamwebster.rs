use std::time::Duration;

use crate::{
    synonym::{merriamwebster::MerriamWebster, Finder, FinderError},
    DictMessage,
};
use actix::{
    prelude::{Actor, Handler},
    AsyncContext, Context, WrapFuture,
};

pub struct MerriamWebsterActor {}

impl MerriamWebsterActor {
    pub fn new() -> Self {
        Self {}
    }
}

/// Declare actor and its context
impl Actor for MerriamWebsterActor {
    type Context = Context<Self>;
}

/// Handler for `WordMessage` message
impl Handler<DictMessage> for MerriamWebsterActor {
    type Result = Result<Vec<String>, Box<dyn std::error::Error + Send>>;

    fn handle(&mut self, msg: DictMessage, ctx: &mut Self::Context) -> Self::Result {
        let time_to_wait = Duration::from_millis(1000);
        ctx.wait(actix::clock::sleep(time_to_wait).into_actor(self));
        match MerriamWebster::new_query(&msg.word).find_synonyms() {
            Ok(res) => Ok(res),
            Err(_) => Err(Box::new(FinderError {})),
        }
    }
}
