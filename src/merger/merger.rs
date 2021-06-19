use std::collections::HashMap;

pub fn count(words: &[String]) -> HashMap<String, i32> {
    let mut counts = HashMap::new();
    words
        .iter()
        .for_each(|w| *counts.entry(w.to_string()).or_insert(0) += 1);
    counts
}
