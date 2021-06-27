use actix::prelude::{Actor, Context, Handler, Recipient, ResponseFuture};

use crate::actors::messages::{DictMessage, WordMessage};
use crate::Counter;

pub struct SynonymsActor {
    dict_addr_vector: Vec<Recipient<DictMessage>>,
}

impl SynonymsActor {
    pub fn new() -> Self {
        let dict_addr_vector = vec![];
        Self { dict_addr_vector }
    }

    pub fn add_dictionary_actor(&mut self, actor: Recipient<DictMessage>) {
        self.dict_addr_vector.push(actor);
    }
}

/// Declare actor and its context
impl Actor for SynonymsActor {
    type Context = Context<Self>;
}

/// Handler for `WordMessage` message
impl Handler<WordMessage> for SynonymsActor {
    type Result = ResponseFuture<Result<Counter, ()>>;
    fn handle(&mut self, msg: WordMessage, _: &mut Context<Self>) -> Self::Result {
        let mut counter = Counter::new();
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
            //let responses = join_all(promises).await?;
            for promise in promises {
                let response = promise.await;
                match response {
                    Ok(res) => {
                        println!("OK");
                        println!("{:?}", res);
                        counter.count(&res.unwrap());
                    }
                    Err(err) => println!("{}", format!("{:?}", err)),
                }
                // println!("{:?}", response);
                // counter.count(response);
            }
            Ok(counter)
        })
    }
}