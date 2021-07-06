//! Modulo con los mensajes que se utilizan para la comunicacion entre actores

use actix::prelude::Message;
use crate::counter::Counter;
use crate::synonym::QueryResponse;
use std::marker::PhantomData;

/// Mensaje de palabra a buscar sobre una pagina
pub struct WordMessage {
    pub word: String,
    pub page_cooldown: u64,
}

impl Message for WordMessage {
    type Result = Result<Counter, ()>;
}

/// Mensaje de palabra a buscar
pub struct DictMessage {
    pub word: String,
    pub page_cooldown: u64,
}

impl Message for DictMessage {
    type Result = Result<QueryResponse, Box<dyn std::error::Error + Send>>;
}

/// Mensaje de palabra a buscar
pub struct RequestMessage<T> {
    pub word: String,
    pub _phantom: PhantomData<T>,
}

impl<T> RequestMessage<T> {
    pub fn new (word: &str) -> Self {
        RequestMessage::<T> {
            word: word.to_owned(),
            _phantom: PhantomData
        }
    }
}

impl<T> Message for RequestMessage<T> {
    type Result = Result<QueryResponse, Box<dyn std::error::Error + Send>>;
}
