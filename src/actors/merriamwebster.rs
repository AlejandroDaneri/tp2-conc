//! Modulo encargado de la busqueda sobre la pagina https://www.merriam-webster.com/thesaurus/

use crate::{actors::messages::DictMessage, actors::requester::RequesterActor, counter::Counter};

use crate::synonym::merriamwebster::MerriamWebster;

use actix::{
    prelude::{Actor, Handler},
    Addr, AsyncContext, Context, ResponseFuture, WrapFuture,
};

use std::time::Duration;

use super::messages::RequestMessage;

/// Actor encargado de la busqueda sobre la pagina https://www.merriam-webster.com/thesaurus/
pub struct MerriamWebsterActor {
    // last_search_time: SystemTime,
    requester: Addr<RequesterActor>,
}

impl MerriamWebsterActor {
    /// Genera un MerriamWebsterActor
    pub fn new(requester: Addr<RequesterActor>) -> Self {
        // let last_search_time = SystemTime::UNIX_EPOCH;
        Self { requester }
    }
}

/// Declare actor and its context
impl Actor for MerriamWebsterActor {
    type Context = Context<Self>;
}

/// Handler for `WordMessage` message
impl Handler<DictMessage> for MerriamWebsterActor {
    type Result = ResponseFuture<Result<Vec<Counter>, Box<dyn std::error::Error + Send>>>;

    fn handle(&mut self, msg: DictMessage, _ctx: &mut Context<Self>) -> Self::Result {
        let words = msg.word.clone();
        let mut counters = Vec::new();
        let requester = self.requester.clone();

        Box::pin(async move {
            for word in words {
                actix::clock::sleep(Duration::from_secs(msg.page_cooldown)).await;
                let promise = requester.send(RequestMessage::<MerriamWebster>::new(&word));
                let response = promise.await;
                match response {
                    Ok(Ok(res)) => {
                        let mut counter = Counter::new(res.word);
                        counter.count(&res.synonyms);
                        counters.push(counter);
                    }
                    Ok(Err(_err)) => todo!(), //TODO: mejorar mensaje de error
                    Err(_err) => todo!(),
                }
            }
            Ok(counters)
        })
    }
}
