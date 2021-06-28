use crate::Counter;
use actix::{prelude::Message, Addr};

/// Main query result
pub struct WordMessage {
    pub word: String,
}

impl Message for WordMessage {
    type Result = Result<Counter, ()>;
}

/// Query per dictionary
pub struct DictMessage {
    pub word: String,
}

impl Message for DictMessage {
    type Result = Result<Vec<String>, Box<dyn std::error::Error + Send>>;
}

/// Query per dictionary
pub struct AddActor {
    pub addr: Addr<Actor>, // un dictionary actor aca
}

impl Message for AddActor {
    type Result = Result<Vec<String>, Box<dyn std::error::Error + Send>>;
}
