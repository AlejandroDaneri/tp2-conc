use super::{Finder, FinderError};

pub struct YourDictionary {
    word: String,
}

impl YourDictionary {
    pub fn new_query(word: &str) -> Self {
        Self {
            word: word.to_owned(),
        }
    }
}

impl Finder for YourDictionary {
    fn url(&self) -> String {
        format!(
            "https://thesaurus.yourdictionary.com/{}",
            self.word.as_str()
        )
    }

    fn parse_body(&self, body: &str) -> Vec<String> {
        let synonyms = body.match_indices("\"synonym-link\"").map(|matched| {
            let synonym_beg = body[matched.0..].find('>').unwrap() + 1 + matched.0;
            let synonym_end = body[synonym_beg..].find('<').unwrap() + synonym_beg;
            body[synonym_beg..synonym_end].to_owned()
        });
        synonyms.collect::<Vec<String>>()
    }
}
