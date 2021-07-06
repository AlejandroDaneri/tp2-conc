//! Modulo encargado de la busqueda sobre la pagina https://www.merriam-webster.com/thesaurus/
use std::time::Duration;

use crate::{
    actors::messages::DictMessage, actors::requester::RequesterActor, counter::Counter,
    synonym::yourdictionary::YourDictionary,
};

use actix::{
    prelude::{Actor, Handler},
    Addr, AsyncContext, Context, ResponseFuture, WrapFuture,
};

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

    fn handle(&mut self, msg: DictMessage, ctx: &mut Context<Self>) -> Self::Result {
        let words = msg.word;
        let mut promises = Vec::new();
        let mut counters = Vec::new();

        for word in words {
            ctx.wait(actix::clock::sleep(Duration::from_secs(msg.page_cooldown)).into_actor(self));

            promises.push(
                self.requester
                    .send(RequestMessage::<YourDictionary>::new(&word)),
            );
        }
        Box::pin(async move {
            for promise in promises {
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
