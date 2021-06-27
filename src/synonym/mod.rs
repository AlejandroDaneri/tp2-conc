pub mod merriamwebster;
pub mod thesaurus;
pub mod yourdictionary;
use reqwest::blocking;

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
    fn url(&self) -> String;
    fn parse_body(&self, body: &str) -> Result<Vec<String>, Box<dyn std::error::Error>>;

    fn find_synonyms(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let url = self.url();
        println!("Url: {}", url);
        let client = blocking::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build();
        match client {
            Ok(client) => {
                let request = client.get(url).send()?;
                let body = request.text()?;
                self.parse_body(body.as_str())
            }
            Err(_) => todo!(),
        }
    }
}
