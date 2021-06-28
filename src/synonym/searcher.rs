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
const MAX_CONCURRENT_REQS: isize = 5;
pub struct Searcher {
    words: Vec<String>,
    conds: Vec<Arc<(Mutex<bool>, Condvar, String)>>,
}
pub enum Provider {
    Thesaurus,
    YourDictionary,
    MerriamWebster,
}
impl Searcher {
    pub fn new(words: Vec<String>) -> Self {
        let mut vec: Vec<Arc<(Mutex<bool>, Condvar, String)>> = Vec::new();
        for i in 0..3 {
            vec.push(Arc::new((Mutex::new(false), Condvar::new(), i.to_string())))
        }
        Self { words, conds: vec }
    }
    pub fn searchs(&self) {
        let log = Arc::new(logger::Logger::new(logger::Level::Debug));

        let mut word_handles = Vec::new();
        let sem = Arc::new(Semaphore::new(MAX_CONCURRENT_REQS));
        for word in self.words.clone() {
            let c_sem = sem.clone();
            let c_log = log.clone();
            let a = self.conds.clone();
            word_handles.push(thread::spawn(move || {
                let mut handles = Vec::new();
                let mut counter = Counter::new(word.clone());
                search_word(c_sem.clone(), word.clone(), &mut handles, &a);

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
    pair: Arc<(Mutex<bool>, Condvar, String)>,
) -> Result<Vec<String>, synonym::FinderError> {
    Box::new(T::new_query(word)).find_synonyms(pair)
}

fn search_word(
    sem: Arc<Semaphore>,
    word: String,
    handles: &mut Vec<thread::JoinHandle<Result<Vec<String>, synonym::FinderError>>>,
    conds: &[Arc<(Mutex<bool>, Condvar, String)>],
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
        let c_conds = conds.to_owned();
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
