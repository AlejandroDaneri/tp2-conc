use std::collections::HashMap;

pub fn count(words: &[String]) -> HashMap<String, i32> {
    let mut counts = HashMap::new();
    words
        .iter()
        .for_each(|w| *counts.entry(w.to_string()).or_insert(0) += 1);
    counts
}

#[cfg(test)]
mod one_vector {
    use crate::counter::count;
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
        let res = count(&v1);

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
        let res = count(&v1);

        let expected = HashMap::<String, i32>::new();

        assert_eq!(expected, res)
    }

    #[test]
    fn duplicates_in_a_vector() {
        let v1 = build_test_vector(&["a", "b", "c", "d", "b"]);
        let res = count(&v1);

        let mut expected = HashMap::<String, i32>::new();
        expected.insert("b".to_string(), 2);
        expected.insert("a".to_string(), 1);
        expected.insert("c".to_string(), 1);
        expected.insert("d".to_string(), 1);

        assert_eq!(expected, res)
    }
}
