//! Modulo encargado de derivar cada busqueda a los diferentes actores de las paginas en concreto

use actix::prelude::{Actor, Context, Handler, Recipient, ResponseFuture};

use crate::actors::messages::{DictMessage, WordMessage};
use crate::{counter::Counter, logger};
/// Actor encargado de derivar cada busqueda a los diferentes actores de las paginas en concreto
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

impl Default for SynonymsActor {
    fn default() -> Self {
        SynonymsActor::new()
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
        let mut counter = Counter::new(msg.word.clone());
        let promises = self
            .dict_addr_vector
            .iter()
            .map(|recipient| {
                let message = DictMessage {
                    word: msg.word.clone(),
                    page_cooldown: msg.page_cooldown,
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
                        counter.count(&res.synonyms);
                    }
                    Ok(Err(err)) => log.error(format!("{:?}", err)), //TODO: mejorar mensaje de error
                    Err(err) => log.error(format!("{:?}", err)),
                }
            }
            Ok(counter)
        })
    }
}
