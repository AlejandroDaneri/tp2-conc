use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::thread;
use std::time::{Duration, SystemTime};

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
}

impl Balancer {
    pub fn new(available_threads: usize, page_cooldown: u64, ids: &[String]) -> Self {
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
        }
    }

    /// Sólo se puede despertar el hilo  con el request más antiguo,
    /// si hay hilos disponibles
    pub fn can_wakeup(&self, id: &str) -> bool {
        let next_finder = self.id_timestamp_heap.peek().unwrap();
        self.available_threads > 0 && id.eq(&next_finder.0.id)
    }

    pub fn take_thread(&mut self) {
        println!("Taken");
        let mut id_ts = self.id_timestamp_heap.pop().unwrap();
        self.sleep_if_necessary(id_ts.0.timestamp);
        id_ts.0.timestamp = SystemTime::now();
        self.id_timestamp_heap.push(id_ts);
        self.available_threads -= 1;
    }

    pub fn release_thread(&mut self) {
        println!("Release");
        self.available_threads += 1;
    }

    pub fn sleep_if_necessary(&self, last_search_time: SystemTime) {
        let now = SystemTime::now();
        let duration = now.duration_since(last_search_time).unwrap();
        if duration.as_secs() < self.page_cooldown {
            thread::sleep(Duration::from_secs(self.page_cooldown - duration.as_secs()))
        }
    }
}
