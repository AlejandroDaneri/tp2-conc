use std::sync::Arc;
use std::thread;

use std_semaphore::Semaphore;

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

pub fn search_word(
    sem: Arc<Semaphore>,
    word: String,
    handles: &mut Vec<thread::JoinHandle<Result<Vec<String>, synonym::FinderError>>>,
) {
    for provider in [
        Provider::MerriamWebster,
        Provider::YourDictionary,
        Provider::Thesaurus,
    ]
    .iter()
    {
        let c_sem = sem.clone();
        let c_word = word.clone();
        handles.push(thread::spawn(move || {
            let _guard = c_sem.access();
            search(&c_word, provider)
        }));
    }
}
