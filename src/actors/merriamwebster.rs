use crate::{
    synonym::{merriamwebster::MerriamWebster, yourdictionary::YourDictionary, Finder},
    DictMessage,
};
use actix::prelude::{Actor, Context, Handler};

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
    type Result = Result<Vec<String>, ()>;

    fn handle(&mut self, msg: DictMessage, _: &mut Context<Self>) -> Self::Result {
        let res = match YourDictionary::new_query(&msg.word).find_synonyms() {
            Ok(syns) => {
                println!("{:?}", syns);
                syns
            }
            Err(error) => {
                println!("error");
                Vec::new()
            }
        };
        Ok(res)
    }
}
