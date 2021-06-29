pub mod merriamwebster;
pub mod searcher;
pub mod thesaurus;
pub mod yourdictionary;
use std::{
    sync::{Arc, Condvar, Mutex},
    thread,
    time::Duration,
};

use reqwest::blocking;

use crate::logger;

const APP_USER_AGENT: &str = "curl/7.68.0";

#[derive(Debug)]
pub struct FinderError;

impl From<reqwest::Error> for FinderError {
    fn from(_error: reqwest::Error) -> Self {
        FinderError
    }
}

impl std::fmt::Display for FinderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error en el buscador de sinónimos")
    }
}

impl std::error::Error for FinderError {}

pub trait Finder {
    fn new_query(word: &str) -> Self
    where
        Self: Sized;
    fn url(&self) -> String;
    fn parse_body(&self, body: &str) -> Vec<String>;

    fn find_synonyms(&self) -> Result<Vec<String>, FinderError> {
        let log = logger::Logger::new(logger::Level::Debug);

        let url = self.url();

        println!("[find_syn] waiting to {:?}", url);
        //esperar hasta que busy sea false

        log.debug(format!("Making request to {:?}", url));

        let client = blocking::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()?;
        let request = client.get(url).send()?;

        // log.debug(format!("sleep  {:?}", url));

        // log.debug(format!(" wake up {:?}", url));

        log.debug(format!("Finish request to {:?}", self.url()));
        let body = request.text()?;
        Ok(self.parse_body(body.as_str()))
    }
}

#[derive(Debug)]
pub enum Provider {
    Thesaurus,
    YourDictionary,
    MerriamWebster,
}
const PROVIDERS: [Provider; 3] = [
    Provider::MerriamWebster,
    Provider::YourDictionary,
    Provider::Thesaurus,
];

impl std::fmt::Display for Provider {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
