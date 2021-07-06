//! Modulo con los mensajes que se utilizan para la comunicacion entre actores

use crate::counter::Counter;
use crate::synonym::QueryResponse;
use actix::prelude::Message;

/// Mensaje de palaba a buscar sobre una pagina
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
