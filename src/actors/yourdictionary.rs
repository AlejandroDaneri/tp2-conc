use crate::DictMessage;
use actix::prelude::{Actor, Context, Handler};

pub struct YourDictionaryActor {}

impl YourDictionaryActor {
    pub fn new() -> Self {
        Self {}
    }
}

/// Declare actor and its context
impl Actor for YourDictionaryActor {
    type Context = Context<Self>;
}

/// Handler for `WordMessage` message
impl Handler<DictMessage> for YourDictionaryActor {
    type Result = Result<Vec<String>, Box<dyn std::error::Error + Send>>;

    fn handle(&mut self, msg: DictMessage, _: &mut Context<Self>) -> Self::Result {
        Ok(vec!["YourDictionary".to_owned(), msg.word])
    }
}
