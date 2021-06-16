pub mod thesaurus;
use reqwest::blocking;

pub trait Finder {
    fn url(&self) -> String;
    fn parse_body(&self, body: &str) -> Result<Vec<String>, Box<dyn std::error::Error>>;

    fn find_synonyms(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let url = self.url();
        println!("Url: {}", url);
        let request = blocking::get(url)?;
        let body = request.text()?;
        self.parse_body(body.as_str())
    }
}
