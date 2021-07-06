use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, SystemTime};

use crate::logger::Logger;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct IdTimestamp {
    timestamp: SystemTime,
    id: String,
}

/// El *balancer* tiene un límite de hilos disponibles y un heap
/// con tuplas id-timestamp, ordenadas según el timestamp, de menor a mayor.
/// De esta forma podemos obtener el id del hilo con fecha más antigua de ejecución.
pub struct Balancer {
    available_threads: usize,
    page_cooldown: u64,
    id_timestamp_heap: BinaryHeap<Reverse<IdTimestamp>>,
    logger: Arc<Logger>,
}

impl Balancer {
    pub fn new(
        available_threads: usize,
        page_cooldown: u64,
        ids: &[String],
        logger: Arc<Logger>,
    ) -> Self {
        let id_timestamps = ids
            .iter()
            .map(|id| {
                Reverse(IdTimestamp {
                    id: id.clone(),
                    timestamp: SystemTime::UNIX_EPOCH,
                })
            })
            .collect::<Vec<Reverse<IdTimestamp>>>();
        let id_timestamp_heap = BinaryHeap::from(id_timestamps);
        Self {
            available_threads,
            id_timestamp_heap,
            page_cooldown,
            logger,
        }
    }

    /// Sólo se puede despertar el hilo  con el request más antiguo,
    /// si hay hilos disponibles
    pub fn can_wakeup(&self, id: &str) -> bool {
        let next_finder = match self.id_timestamp_heap.peek() {
            Some(next_finder) => next_finder,
            _ => {
                self.logger.error("Error peak wake up".to_string());
                panic!()
            }
        };
        self.available_threads > 0 && id.eq(&next_finder.0.id)
    }

    pub fn take_thread(&mut self) {
        let mut id_ts = match self.id_timestamp_heap.pop() {
            Some(it) => it,
            _ => {
                self.logger.error("Error poping thread heap".to_string());
                panic!()
            }
        };
        self.logger.info(format!("{:?} taken", id_ts.0.id));
        self.sleep_if_necessary(id_ts.0.timestamp);
        id_ts.0.timestamp = SystemTime::now();
        self.id_timestamp_heap.push(id_ts);
        self.available_threads -= 1;
    }

    pub fn release_thread(&mut self) {
        self.logger.info("Release".to_string());
        self.available_threads += 1;
    }

    pub fn sleep_if_necessary(&self, last_search_time: SystemTime) {
        let now = SystemTime::now();
        let duration = match now.duration_since(last_search_time) {
            Ok(duration) => duration,
            _ => {
                self.logger.error("Error calculating duration".to_string());
                panic!()
            }
        };
        if duration.as_secs() < self.page_cooldown {
            thread::sleep(Duration::from_secs(self.page_cooldown - duration.as_secs()))
        }
    }
}
