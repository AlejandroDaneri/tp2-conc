pub mod merriamwebster;
pub mod thesaurus;
pub mod yourdictionary;
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

pub trait Finder {
    fn new_query(word: &str) -> Self
    where
        Self: Sized;
    fn url(&self) -> String;
    fn parse_body(&self, body: &str) -> Vec<String>;

    fn find_synonyms(&self) -> Result<Vec<String>, FinderError> {
        let log = logger::Logger::new(logger::Level::Debug);

        let url = self.url();
        log.debug(format!("Making request to {:?}", url));
        let client = blocking::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()?;
        let request = client.get(url).send()?;
        let body = request.text()?;
        Ok(self.parse_body(body.as_str()))
    }
}
