use crate::counter::Counter;
use crate::logger::Logger;
use crate::synonym::balancer::Balancer;
use crate::synonym::finder_executor::FinderExecutor;
use crate::synonym::merriamwebster::MerriamWebster;
use crate::synonym::thesaurus::Thesaurus;
use crate::synonym::yourdictionary::YourDictionary;
use std::collections::HashMap;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::thread::JoinHandle;

pub struct Searcher {
    words: Vec<String>,
}

impl Searcher {
    /// Crea un nuevo Searcher
    pub fn new(words: Vec<String>) -> Self {
        Self { words }
    }

    pub fn searchs(&self, page_cooldown: u64, max_conc_reqs: usize, logger: Arc<Logger>) {
        let ids = vec![
            "MerriamWebster".to_string(),
            "Thesaurus".to_string(),
            "YourDictionary".to_string(),
        ];
        let balancer = Arc::new(Mutex::new(Balancer::new(
            max_conc_reqs,
            page_cooldown,
            &ids,
            logger,
        )));
        let cond_var = Arc::new(Condvar::new());
        let mut handlers = vec![];

        macro_rules! push_executor {
            ($executor:ty ) => {{
                let executor = FinderExecutor::<$executor>::new(balancer.clone(), cond_var.clone());
                let words = self.words.clone();
                let balancer_enabled = max_conc_reqs != 0;
                handlers.push(thread::spawn(move || {
                    executor.execute(&words, balancer_enabled)
                }));
            }};
        }

        push_executor!(Thesaurus);
        push_executor!(MerriamWebster);
        push_executor!(YourDictionary);

        let mut result = HashMap::<String, Counter>::new();
        handlers
            .into_iter()
            .map(JoinHandle::join)
            .flatten()
            .flatten()
            .for_each(|counter| {
                let word = counter.word.clone();
                let prev_counter = result
                    .entry(word.clone())
                    .or_insert_with(|| Counter::new(word));
                prev_counter.merge(&counter)
            });
        result.values().for_each(|counter| {
            println!("{}", counter);
        });
    }
}
