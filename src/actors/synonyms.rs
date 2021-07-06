//! Modulo encargado de derivar cada busqueda a los diferentes actores de las paginas en concreto

use actix::prelude::{Actor, Context, Handler, Recipient, ResponseFuture};

use crate::actors::messages::{DictMessage, WordMessage};
use crate::counter::Counter;
use crate::logger;
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
    type Result = ResponseFuture<Result<Vec<Counter>, ()>>;
    fn handle(&mut self, msg: WordMessage, _: &mut Context<Self>) -> Self::Result {
        let mut counters: Vec<Counter> = Vec::new();
        let mut pages_promises = Vec::new();
        let _c_msg = msg.clone();
        pages_promises = self
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
            for promise in pages_promises {
                let response = promise.await;
                match response {
                    Ok(Ok(res)) => {
                        if !counters.is_empty() {
                            for (i, counter_res) in res.iter().enumerate() {
                                counters[i].merge(counter_res);
                            }
                        } else {
                            counters = res
                        }
                    }
                    Ok(Err(err)) => log.error(format!("{:?}", err)), //TODO: mejorar mensaje de error
                    Err(err) => log.error(format!("{:?}", err)),
                }
            }
            Ok(counters)
        })
    }
}
