use super::Finder;

pub struct MerriamWebster {
    word: String,
}

impl Finder for MerriamWebster {
    fn new_query(word: &str) -> Self {
        Self {
            word: word.to_owned(),
        }
    }

    fn url(&self) -> String {
        format!(
            "https://www.merriam-webster.com/thesaurus/{}",
            self.word.as_str()
        )
    }

    fn parse_body(&self, body: &str) -> Vec<String> {
        let from = match body.find("thes-list syn-list") {
            Some(v) => v,
            None => {0},
        };
        let to = match body.find("thes-list rel-list") {
            Some(v) => v,
            None => {0},
        };
        let body = &body[from..to];
        let synonyms = body
            .match_indices("<a class=\"\" href=\"/thesaurus/")
            .map(|matched| {
                let synonym_beg = match body[matched.0..].find('>') {
                    Some(v) => v + 1 + matched.0,
                    None => {0},
                };
                let synonym_end = match body[synonym_beg..].find('<') {
                    Some(v) => v + synonym_beg,
                    None => {0}
                };
                body[synonym_beg..synonym_end].to_owned()
            });
        synonyms.collect::<Vec<String>>()
    }
}
