use super::{
    merriamwebster::MerriamWebster, thesaurus::Thesaurus, yourdictionary::YourDictionary, Finder,
};

use super::super::synonym;
pub enum Provider {
    Thesaurus,
    YourDictionary,
    MerriamWebster,
}
pub fn search(word: &str, provider: &Provider) -> Result<Vec<String>, synonym::FinderError> {
    match provider {
        Provider::Thesaurus => _search::<Thesaurus>(&word),
        Provider::MerriamWebster => _search::<MerriamWebster>(&word),
        Provider::YourDictionary => _search::<YourDictionary>(&word),
    }
}

fn _search<T: 'static + Finder + Send>(word: &str) -> Result<Vec<String>, synonym::FinderError> {
    Box::new(T::new_query(word)).find_synonyms()
}
