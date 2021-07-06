//! Modulo encargado de la busqueda sobre la pagina https://www.thesaurus.com/browse/

use crate::counter::Counter;

use crate::{actors::messages::DictMessage, synonym::thesaurus::Thesaurus};
use actix::{
    prelude::{Actor, Handler},
    Addr,
};
use actix::{AsyncContext, Context, ResponseFuture, WrapFuture};

use std::time::{Duration, SystemTime};

use super::{messages::RequestMessage, requester::RequesterActor};

/// Actor encargado de la busqueda sobre la pagina https://www.thesaurus.com/browse/

pub struct ThesaurusActor {
    // last_search_time: SystemTime,
    requester: Addr<RequesterActor>,
}

impl ThesaurusActor {
    /// Genera un ThesaurusActor
    pub fn new(requester: Addr<RequesterActor>) -> Self {
        let _last_search_time = SystemTime::UNIX_EPOCH;
        Self { requester }
    }
}

/// Declare actor and its context
impl Actor for ThesaurusActor {
    type Context = Context<Self>;
}

/// Handler for `WordMessage` message
impl Handler<DictMessage> for ThesaurusActor {
    type Result = ResponseFuture<Result<Vec<Counter>, Box<dyn std::error::Error + Send>>>;

    fn handle(&mut self, msg: DictMessage, _ctx: &mut Context<Self>) -> Self::Result {
        let words = msg.word.clone();
        let mut counters = Vec::new();
        let requester = self.requester.clone();

        Box::pin(async move {
            for word in words {
                actix::clock::sleep(Duration::from_secs(msg.page_cooldown)).await;
                let promise = requester.send(RequestMessage::<Thesaurus>::new(&word));
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
