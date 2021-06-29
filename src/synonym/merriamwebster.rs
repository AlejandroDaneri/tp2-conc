//! Encargado de la busqueda sobre la pagina https://www.merriam-webster.com/thesaurus/

use super::Finder;
/// Encargado de la busqueda sobre la pagina https://www.merriam-webster.com/thesaurus/

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
    /// Hace el parseo del contenido de la pagina
    fn parse_body(&self, body: &str) -> Vec<String> {
        let from = body.find("thes-list syn-list").unwrap_or(0);
        let to = body.find("thes-list rel-list").unwrap_or(0);
        let body = &body[from..to];
        let synonyms = body
            .match_indices("<a class=\"\" href=\"/thesaurus/")
            .map(|matched| {
                let synonym_beg = body[matched.0..].find('>').unwrap_or(0) + 1 + matched.0;
                let synonym_end = body[synonym_beg..].find('<').unwrap_or(0) + synonym_beg;
                body[synonym_beg..synonym_end].to_owned()
            });
        synonyms.collect::<Vec<String>>()
    }
}
