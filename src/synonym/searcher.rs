use std::sync::Arc;
use std::sync::Condvar;
use std::sync::Mutex;
use std::thread;

use std_semaphore::Semaphore;

use crate::counter::Counter;
use crate::logger;

use super::{
    merriamwebster::MerriamWebster, thesaurus::Thesaurus, yourdictionary::YourDictionary, Finder,
};

use super::super::synonym;
const MAX_CONCURRENT_REQS: isize = 2;
pub struct Searcher {
    words: Vec<String>,
    conds: Vec<Arc<(Mutex<()>, Condvar)>>,
}
pub enum Provider {
    Thesaurus,
    YourDictionary,
    MerriamWebster,
}
impl Searcher {
    pub fn new(words: Vec<String>) -> Self {
        Self {
            words,
            conds: vec![Arc::new((Mutex::new(()), Condvar::new())); 3],
        }
    }

    // pub fn search(
    //     &self,
    //     word: &str,
    //     provider: &Provider,
    // ) -> Result<Vec<String>, synonym::FinderError> {
    //     match provider {
    //         Provider::Thesaurus => _search::<Thesaurus>(&word, self.conds[0]),
    //         Provider::MerriamWebster => _search::<MerriamWebster>(&word, self.conds[1]),
    //         Provider::YourDictionary => _search::<YourDictionary>(&word, self.conds[2]),
    //     }
    // }

    pub fn searchs(&self) {
        let log = Arc::new(logger::Logger::new(logger::Level::Debug));

        let mut word_handles = Vec::new();
        let sem = Arc::new(Semaphore::new(MAX_CONCURRENT_REQS));
        let c_conds = Arc::new(self.conds.clone());
        for word in self.words.clone() {
            let c_sem = sem.clone();
            let c_log = log.clone();
            word_handles.push(thread::spawn(move || {
                let mut handles = Vec::new();
                let mut counter = Counter::new(word.clone());
                search_word(c_sem.clone(), word.clone(), &mut handles, c_conds.clone());

                let results = handles.into_iter().map(|handle| handle.join());

                results
                    .map(|result| {
                        if result.is_err() {
                            c_log.warn(format!("Problem getting synonyms: {:?}", result));
                        }
                        result
                    })
                    .flatten()
                    .flatten()
                    .for_each(|syn_list| {
                        counter.count(&syn_list);
                    });

                c_log.info(format!("COUNT: {:?}", counter.get_counter()));
            }));
        }
        for thread in word_handles {
            thread
                .join()
                .map_err(|err| log.error(format!("Word join error {:?}", err)))
                .ok();
        }
    }
}
fn _search<T: 'static + Finder + Send>(
    word: &str,
    pair: Arc<(Mutex<()>, Condvar)>,
) -> Result<Vec<String>, synonym::FinderError> {
    Box::new(T::new_query(word)).find_synonyms()
}

fn search_word(
    sem: Arc<Semaphore>,
    word: String,
    handles: &mut Vec<thread::JoinHandle<Result<Vec<String>, synonym::FinderError>>>,
    conds: Vec<Arc<(Mutex<()>, Condvar)>>,
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
        let c_conds = conds.clone();
        handles.push(thread::spawn(move || {
            let _guard = c_sem.access();
            match provider {
                Provider::Thesaurus => _search::<Thesaurus>(&c_word, c_conds[0].clone()),
                Provider::MerriamWebster => _search::<MerriamWebster>(&c_word, c_conds[1].clone()),
                Provider::YourDictionary => _search::<YourDictionary>(&c_word, c_conds[2].clone()),
            }
        }));
    }
}
