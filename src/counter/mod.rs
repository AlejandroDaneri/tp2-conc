use std::collections::HashMap;
#[derive(Clone, Debug)]
pub struct Counter {
    word: String,
    count: HashMap<String, i32>,
}

impl Counter {
    pub fn new(word: String) -> Self {
        Self {
            word,
            count: HashMap::<String, i32>::new(),
        }
    }

    pub fn count(&mut self, words: &[String]) -> HashMap<String, i32> {
        words
            .iter()
            .for_each(|w| *self.count.entry(w.to_string()).or_insert(0) += 1);
        self.count.clone()
    }

    pub fn get_counter(&self) -> &Self {
        self // TODO: mejorar el return para que cumpla con lo pedido
    }
}

#[cfg(test)]
mod one_vector {
    use crate::counter::Counter;
    use std::collections::HashMap;

    fn build_test_vector(elements: &[&str]) -> Vec<String> {
        let mut result = Vec::<String>::new();
        for word in elements.iter() {
            result.push(word.to_string());
        }
        result
    }

    #[test]
    fn count_words() {
        let v1 = build_test_vector(&["a", "b", "c", "d"]);

        let mut counter = Counter::new();
        let res = counter.count(&v1);

        let mut expected = HashMap::<String, i32>::new();
        expected.insert("b".to_string(), 1);
        expected.insert("a".to_string(), 1);
        expected.insert("c".to_string(), 1);
        expected.insert("d".to_string(), 1);

        assert_eq!(expected, res)
    }

    #[test]
    fn empty_vector() {
        let v1 = build_test_vector(&[]);
        let mut counter = Counter::new();
        let res = counter.count(&v1);

        let expected = HashMap::<String, i32>::new();

        assert_eq!(expected, res)
    }

    #[test]
    fn duplicates_in_a_vector() {
        let v1 = build_test_vector(&["a", "b", "c", "d", "b"]);
        let mut counter = Counter::new();
        let res = counter.count(&v1);

        let mut expected = HashMap::<String, i32>::new();
        expected.insert("b".to_string(), 2);
        expected.insert("a".to_string(), 1);
        expected.insert("c".to_string(), 1);
        expected.insert("d".to_string(), 1);

        assert_eq!(expected, res)
    }
}
