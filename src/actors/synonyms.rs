use actix::prelude::{Actor, Handler, Recipient, ResponseFuture};
use actix::SyncContext;

use crate::actors::messages::{DictMessage, WordMessage};
use crate::{logger, Counter};

use super::messages::AddActor;

pub struct SynonymsActor {
    dict_addr_vector: Vec<Recipient<DictMessage>>,
}

impl SynonymsActor {
    pub fn new() -> Self {
        let dict_addr_vector = vec![];
        Self { dict_addr_vector }
    }
}

/// Declare actor and its context
impl Actor for SynonymsActor {
    type Context = SyncContext<Self>;
}

impl Handler<AddActor> for SynonymsActor {
    type Result = ();

    fn handle(&mut self, msg: AddActor, _: &mut SyncContext<Self>) -> Self::Result {
        self.dict_addr_vector.push(msg.addr);
    }
}
/// Handler for `WordMessage` message
impl Handler<WordMessage> for SynonymsActor {
    type Result = ResponseFuture<Result<Counter, ()>>;
    fn handle(&mut self, msg: WordMessage, _: &mut SyncContext<Self>) -> Self::Result {
        let mut counter = Counter::new(msg.word.clone());
        let promises = self
            .dict_addr_vector
            .iter()
            .map(|recipient| {
                let message = DictMessage {
                    word: msg.word.clone(),
                };
                recipient.send(message)
            })
            .collect::<Vec<_>>();
        Box::pin(async move {
            let log = logger::Logger::new(logger::Level::Debug);
            //let responses = join_all(promises).await?;
            for promise in promises {
                let response = promise.await;
                match response {
                    Ok(Ok(res)) => {
                        counter.count(&res);
                    }
                    Ok(Err(err)) => log.error(format!("{:?}", err)), //TODO: mejorar mensaje de error
                    Err(err) => log.error(format!("{:?}", err)),
                }
            }
            Ok(counter)
        })
    }
}
