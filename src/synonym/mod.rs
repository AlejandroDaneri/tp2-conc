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

pub trait Finder {
    fn new_query(word: &str) -> Self
    where
        Self: Sized;
    fn url(&self) -> String;
    fn parse_body(&self, body: &str) -> Vec<String>;

    fn find_synonyms(
        &self,
        pair: Arc<(Mutex<bool>, Condvar, String)>,
    ) -> Result<Vec<String>, FinderError> {
        let log = logger::Logger::new(logger::Level::Debug);

        let url = self.url();

        println!("[find_syn] waiting to {:?}", url);
        let (lock, cvar, str) = &*pair;
        //esperar hasta que busy sea false
        let mut busy = cvar
            .wait_while(lock.lock().unwrap(), |busy| {
                log.debug(format!("CVAR {:?}, {:?}, {:?}", url, busy, str));
                *busy
            })
            .unwrap();
        *busy = true;
        log.debug(format!("Making request to {:?}", url));

        let client = blocking::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()?;
        let request = client.get(url.clone()).send()?;

        log.debug(format!("sleep  {:?}", url.clone()));
        thread::sleep(Duration::from_millis(3000));
        log.debug(format!(" wake up {:?}", url.clone()));
        *busy = false;
        cvar.notify_all();
        log.debug(format!("Finish request to {:?}", self.url()));
        let body = request.text()?;
        Ok(self.parse_body(body.as_str()))
    }
}
