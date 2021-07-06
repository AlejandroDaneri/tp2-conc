use crate::counter::Counter;
use crate::synonym::balancer::Balancer;
use crate::synonym::{Finder, FinderError, QueryResponse};
use std::marker::PhantomData;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::thread::JoinHandle;

pub struct FinderExecutor<T: Finder> {
    balancer: Arc<Mutex<Balancer>>,
    cond_var: Arc<Condvar>,
    _phantom: PhantomData<T>,
}

/// Ejecuta Finder::find_synonyms con los mecanismos de sincronizacion
impl<T: 'static + Finder + Send> FinderExecutor<T> {
    pub fn new(balancer: Arc<Mutex<Balancer>>, cond_var: Arc<Condvar>) -> Self {
        let _phantom = PhantomData;
        Self {
            balancer,
            cond_var,
            _phantom,
        }
    }

    pub fn execute(&self, words: &[String], balancer_enabled: bool) -> Vec<Counter> {
        let handles;
        if balancer_enabled {
            handles = self.execute_sync_threaded(words)
        } else {
            // Si no tengo límite de hilos, proceso todos juntos
            handles = self.execute_full_threaded(words)
        }
        handles
            .into_iter()
            .map(|handle| handle.join())
            .flatten()
            .flatten()
            .map(|query_res| {
                let mut counter = Counter::new(query_res.word.clone());
                counter.count(&query_res.synonyms);
                counter
            })
            .collect()
    }

    fn execute_full_threaded(
        &self,
        words: &[String],
    ) -> Vec<JoinHandle<Result<QueryResponse, FinderError>>> {
        words
            .iter()
            .map(|word| {
                let query = T::new_query(word);
                thread::spawn(move || query.find_synonyms())
            })
            .collect::<Vec<JoinHandle<_>>>()
    }

    fn execute_sync_threaded(
        &self,
        words: &[String],
    ) -> Vec<JoinHandle<Result<QueryResponse, FinderError>>> {
        words
            .iter()
            .map(|word| {
                let query = T::new_query(word);
                let query_id = T::get_id();
                let balancer_c = self.balancer.clone();
                let cond_var_c = self.cond_var.clone();
                thread::spawn(move || {
                    {
                        let mut balancer_guard = cond_var_c
                            .wait_while(
                                match balancer_c.lock() {
                                    Ok(it) => it,
                                    _ => unreachable!(),
                                },
                                |bal| !bal.can_wakeup(&query_id),
                            )
                            .map_err(|_| FinderError)?;
                        balancer_guard.take_thread();
                    }
                    let res = query.find_synonyms();
                    balancer_c.lock().map_err(|_| FinderError)?.release_thread();
                    cond_var_c.notify_all();
                    res
                })
            })
            .collect::<Vec<JoinHandle<_>>>()
    }
}
