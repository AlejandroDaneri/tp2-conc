use super::Finder;

pub struct YourDictionary {
    word: String,
}

impl Finder for YourDictionary {
    fn new_query(word: &str) -> Self {
        Self {
            word: word.to_owned(),
        }
    }

    fn url(&self) -> String {
        format!(
            "https://thesaurus.yourdictionary.com/{}",
            self.word.as_str()
        )
    }

    fn parse_body(&self, body: &str) -> Vec<String> {
        let synonyms = body.match_indices("\"synonym-link\"").map(|matched| {
            let synonym_beg = body[matched.0..].find('>').unwrap_or(0) + 1 + matched.0;
            let synonym_end = body[synonym_beg..].find('<').unwrap_or(0) + synonym_beg;
            body[synonym_beg..synonym_end].to_owned()
        });
        synonyms.collect::<Vec<String>>()
    }
}
