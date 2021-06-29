//! Modulo para la coordinacion de las busquedas en las diversas paginas

use std::sync::Arc;
use std::sync::Condvar;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, SystemTime};

use std_semaphore::Semaphore;

use crate::counter::Counter;
use crate::logger;
use crate::synonym::FinderError;

use super::{
    merriamwebster::MerriamWebster, thesaurus::Thesaurus, yourdictionary::YourDictionary, Finder,
    Provider, PROVIDERS,
};

use super::super::synonym;
/// Se utilza para la coordinacion de las busquedas en las diversas paginas

pub struct Searcher {
    words: Vec<String>,
    conds: Vec<Arc<(Mutex<bool>, Condvar, String)>>,
}

#[allow(clippy::mutex_atomic)]
impl Searcher {
    /// Crea un nuevo Searcher
    pub fn new(words: Vec<String>) -> Self {
        let mut vec: Vec<Arc<(Mutex<bool>, Condvar, String)>> = Vec::new();
        for prov in PROVIDERS.iter() {
            vec.push(Arc::new((
                Mutex::new(false),
                Condvar::new(),
                prov.to_string(),
            )))
        }
        Self { words, conds: vec }
    }
    /// Hace la busqueda sobre todas las paginas

    pub fn searchs(&self, page_cooldown: u64, max_conc_reqs: isize) {
        let log = Arc::new(logger::Logger::new(logger::Level::Debug));
        let sem = Arc::new(Semaphore::new(max_conc_reqs));

        let thes_last_time = Arc::new(Mutex::new(SystemTime::UNIX_EPOCH));
        let your_dict_last_time = Arc::new(Mutex::new(SystemTime::UNIX_EPOCH));
        let merriam_last_time = Arc::new(Mutex::new(SystemTime::UNIX_EPOCH));

        let word_handles = self
            .words
            .clone()
            .into_iter()
            .map(|word| {
                let c_sem = sem.clone();
                let c_log = log.clone();
                let providers_conds = self.conds.clone();
                let c_thes_last_time = thes_last_time.clone();
                let c_your_dict_last_time = your_dict_last_time.clone();
                let c_merriam_last_time = merriam_last_time.clone();
                thread::spawn(move || {
                    let mut counter = Counter::new(word.clone());
                    let handles = PROVIDERS.iter().map(|provider| {
                        let mut c_thes_last_time = c_thes_last_time.clone();
                        let mut c_your_dict_last_time = c_your_dict_last_time.clone();
                        let mut c_merriam_last_time = c_merriam_last_time.clone();
                        let c_sem = c_sem.clone();
                        let c_word = word.clone();
                        let c_conds = providers_conds.to_owned();
                        thread::spawn(move || {
                            let _guard = c_sem.access();
                            match provider {
                                Provider::Thesaurus => _search::<Thesaurus>(
                                    &c_word,
                                    c_conds[0].clone(),
                                    &mut c_thes_last_time,
                                    page_cooldown,
                                ),
                                Provider::MerriamWebster => _search::<MerriamWebster>(
                                    &c_word,
                                    c_conds[1].clone(),
                                    &mut c_merriam_last_time,
                                    page_cooldown,
                                ),
                                Provider::YourDictionary => _search::<YourDictionary>(
                                    &c_word,
                                    c_conds[2].clone(),
                                    &mut c_your_dict_last_time,
                                    page_cooldown,
                                ),
                            }
                        })
                    });
                    // .collect::<Vec<JoinHandle<_>>>();

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

                    counter.print_counter();
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
/// Hace la busqueda sobre una pagina en especifico
fn _search<T: Finder + Send>(
    word: &str,
    pair: Arc<(Mutex<bool>, Condvar, String)>,
    last_search_time: &mut Arc<Mutex<SystemTime>>,
    page_cooldown: u64,
) -> Result<Vec<String>, synonym::FinderError> {
    let log = logger::Logger::new(logger::Level::Debug);
    let (lock, cvar, str) = &*pair;
    let now = SystemTime::now();
    log.debug(format!(
        "[{:?} last search {:?}: {:?}",
        now, str, last_search_time
    ));

    let mut busy = cvar
        .wait_while(lock.lock().map_err(|_| FinderError)?, |busy| {
            log.debug(format!("CVAR {:?}, {:?}", busy, str));
            *busy
        })
        .map_err(|_| FinderError)?;
    *busy = true;
    let mut last_time = last_search_time.lock().map_err(|_| FinderError)?;
    let res = Box::new(T::new_query(word)).find_synonyms();
    let duration = match now.duration_since(*last_time) {
        Ok(duration) => duration,
        _ => unreachable!(),
    };
    *last_time = now;
    if duration.as_secs() < page_cooldown {
        log.debug(format!("Waiting {:?}", page_cooldown - duration.as_secs()));
        thread::sleep(Duration::from_secs(page_cooldown - duration.as_secs()));
    }
    *busy = false;
    cvar.notify_all();
    res
}
