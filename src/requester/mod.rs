//! Se encarga de llevar el conteo de las palabras que se les pasa

const APP_USER_AGENT: &str = "curl/7.68.0";
use reqwest::blocking;

#[derive(Clone, Debug)]
pub struct Requester {}

impl Requester {
    pub fn make_request(url: String) -> Result<String, reqwest::Error> {
        let client = blocking::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()?;
        client.get(url).send()?.text()
    }
}
