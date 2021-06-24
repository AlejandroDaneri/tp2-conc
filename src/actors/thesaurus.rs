use crate::DictMessage;
use actix::prelude::{Actor, Context, Handler};

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
    type Result = Result<Vec<String>, ()>;

    fn handle(&mut self, msg: DictMessage, _: &mut Context<Self>) -> Self::Result {
        Ok(vec!["Thesaurus".to_owned(), msg.word])
    }
}
