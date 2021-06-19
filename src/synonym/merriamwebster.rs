use super::Finder;

pub struct MerriamWebster {
    word: String
}

impl MerriamWebster {
    pub fn new_query(word: &str) -> Self {
        Self{word: word.to_owned()}
    }
}

impl Finder for MerriamWebster {
    fn url(&self) -> String {
        format!("https://www.merriam-webster.com/thesaurus/{}", self.word.as_str())
    }

    fn parse_body(&self, body: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let from = body.find("thes-list syn-list").unwrap();
        let to = body.find("thes-list rel-list").unwrap();
        let body = &body[from..to];
        let synonyms = body.match_indices("<a class=\"\" href=\"/thesaurus/").map(|matched| {
    		let synonym_beg = body[matched.0 ..].find('>').unwrap() + 1 + matched.0;
    		let synonym_end = body[synonym_beg ..].find('<').unwrap() + synonym_beg;
        	body[synonym_beg .. synonym_end].to_owned()
        });
        Ok(synonyms.collect::<Vec<String>>())
    }
}
