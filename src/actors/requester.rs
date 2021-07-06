//!Actor encargado de hacer las request a las paginas

use crate::synonym::Finder;
use crate::{actors::messages::DictMessage, requester::Requester};
use actix::{
    prelude::{Actor, Handler},
    SyncContext,
};

use super::messages::RequestMessage;

/// Actor encargado de hacer las request a las paginas

pub struct RequesterActor {}

impl RequesterActor {
    /// Genera un ThesaurusActor
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for RequesterActor {
    fn default() -> Self {
        RequesterActor::new()
    }
}

/// Declare actor and its context
impl Actor for RequesterActor {
    type Context = SyncContext<Self>;
}

/// Handler for `Request` message
impl Handler<RequestMessage> for RequesterActor {
    type Result = Result<String, Box<dyn std::error::Error + Send>>;

    fn handle(&mut self, msg: DictMessage, _: &mut SyncContext<Self>) -> Self::Result {
        Requester::make_request(msg.url)
    }
}
