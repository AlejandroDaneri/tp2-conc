//! Modulo con los mensajes que se utilizan para la comunicacion entre actores

use crate::counter::Counter;
use crate::synonym::QueryResponse;
use actix::prelude::Message;
use std::marker::PhantomData;

/// Mensaje de palabra a buscar sobre una pagina
pub struct WordMessage {
    pub word: Vec<String>,
    pub page_cooldown: u64,
}

impl Message for WordMessage {
    type Result = Result<Vec<Counter>, ()>;
}

impl Clone for WordMessage {
    fn clone(&self) -> Self {
        Self {
            word: self.word.clone(),
            page_cooldown: self.page_cooldown,
        }
    }
}

/// Mensaje de palabra a buscar
pub struct DictMessage {
    pub word: Vec<String>,
    pub page_cooldown: u64,
}

impl Message for DictMessage {
    type Result = Result<Vec<Counter>, Box<dyn std::error::Error + Send>>;
}

/// Mensaje de palabra a buscar
pub struct RequestMessage<T> {
    pub word: String,
    pub _phantom: PhantomData<T>,
}

impl<T> RequestMessage<T> {
    pub fn new(word: &str) -> Self {
        RequestMessage::<T> {
            word: word.to_owned(),
            _phantom: PhantomData,
        }
    }
}

impl<T> Message for RequestMessage<T> {
    type Result = Result<QueryResponse, Box<dyn std::error::Error + Send>>;
}
