use crate::synonym::Finder;
use crate::synonym::FinderError;
use crate::synonym::QueryResponse;
use actix::Actor;
use actix::SyncContext;
use actix::Handler;

use crate::actors::messages::RequestMessage;

pub struct RequesterActor;

impl RequesterActor {
    /// Genera un RequesterActor
    pub fn new() -> Self {
        Self{}
    }
}

impl Default for RequesterActor {
    fn default() -> Self {
        RequesterActor::new()
    }
}

impl Actor for RequesterActor {
    type Context = SyncContext<Self>;
}

/// Handler for `WordMessage` message
impl<T: Finder> Handler<RequestMessage<T>> for RequesterActor {
    type Result = Result<QueryResponse, Box<dyn std::error::Error + Send>>;

    fn handle(&mut self, msg: RequestMessage<T>, _: &mut SyncContext<Self>) -> Self::Result {
        if let Ok(res) = T::new_query(&msg.word).find_synonyms() {
            Ok(res)
        } else {
            Err(Box::new(FinderError {}))
        }
    }
}
