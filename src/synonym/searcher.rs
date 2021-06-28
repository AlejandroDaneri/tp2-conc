use std::sync::Arc;
use std::sync::Condvar;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;

use std_semaphore::Semaphore;

use crate::counter::Counter;
use crate::logger;

use super::{
    Provider,
    PROVIDERS,
    merriamwebster::MerriamWebster, thesaurus::Thesaurus, yourdictionary::YourDictionary, Finder,
};

use super::super::synonym;
const MAX_CONCURRENT_REQS: isize = 5;
pub struct Searcher {
    words: Vec<String>,
    conds: Vec<Arc<(Mutex<bool>, Condvar, String)>>,
}

#[allow(clippy::mutex_atomic)]
impl Searcher {
    pub fn new(words: Vec<String>) -> Self {
        let mut vec: Vec<Arc<(Mutex<bool>, Condvar, String)>> = Vec::new();
        for prov in PROVIDERS.iter() {
            vec.push(Arc::new((Mutex::new(false), Condvar::new(), prov.to_string())))
        }
        Self { words, conds: vec }
    }

    pub fn searchs(&self) {
        let log = Arc::new(logger::Logger::new(logger::Level::Debug));
        let sem = Arc::new(Semaphore::new(MAX_CONCURRENT_REQS));

        let word_handles = self
            .words
            .clone()
            .into_iter()
            .map(|word| {
                let c_sem = sem.clone();
                let c_log = log.clone();
                let a = self.conds.clone();
                thread::spawn(move || {
                    let mut counter = Counter::new(word.clone());
                    let handles = launch_searchers(c_sem.clone(), word.clone(), &a);

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
                })
            })
            .collect::<Vec<JoinHandle<_>>>();

        for thread in word_handles {
            thread
                .join()
                .map_err(|err| log.error(format!("Word join error {:?}", err)))
                .ok();
        }
    }
}
fn _search<T: Finder + Send>(
    word: &str,
    pair: Arc<(Mutex<bool>, Condvar, String)>,
) -> Result<Vec<String>, synonym::FinderError> {
    Box::new(T::new_query(word)).find_synonyms(pair)
}

fn launch_searchers(
    sem: Arc<Semaphore>,
    word: String,
    conds: &[Arc<(Mutex<bool>, Condvar, String)>],
) -> Vec<thread::JoinHandle<Result<Vec<String>, synonym::FinderError>>> {
    PROVIDERS
    .iter()
    .map(|provider| {
        let c_sem = sem.clone();
        let c_word = word.clone();
        let c_conds = conds.to_owned();
        thread::spawn(move || {
            let _guard = c_sem.access();
            match provider {
                Provider::Thesaurus => _search::<Thesaurus>(&c_word, c_conds[0].clone()),
                Provider::MerriamWebster => _search::<MerriamWebster>(&c_word, c_conds[1].clone()),
                Provider::YourDictionary => _search::<YourDictionary>(&c_word, c_conds[2].clone()),
            }
        })
    })
    .collect()
}
