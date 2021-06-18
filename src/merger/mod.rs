pub struct Merger {} //TODO: change to mod

impl Merger {
    pub fn merge(first: &Vec<String>, second: &Vec<String>) -> Vec<String> {
        [&first[..], &second[..]].concat()
    }
}

#[cfg(test)]
mod happy_path {
    fn assert_contains(result: &Vec<String>, expected: &[&str]) {
        for word in expected.iter() {
            assert!(result.contains(&word.to_string()));
        }
    }

    fn build_test_vector(elements: &[&str]) -> Vec<String> {
        let mut result = Vec::<String>::new();
        for word in elements.iter() {
            result.push(word.to_string());
        }
        result
    }

    #[test]
    fn merge_with_two_wellformed_arrays() {
        use crate::merger::Merger;
        let v1 = build_test_vector(&["as", "bs"]);
        let v2 = build_test_vector(&["cs", "ds"]);

        let res = Merger::merge(&v1, &v2);

        assert_contains(&res, &["as", "bs", "cs", "ds"]);
        assert_eq!(res.len(), 4);
    }

    #[test]
    fn merge_with_empty_array() {
        use crate::merger::Merger;

        let v1 = build_test_vector(&["as", "bs"]);
        let v2 = build_test_vector(&[]);

        let res = Merger::merge(&v1, &v2);
        assert_contains(&res, &["as", "bs"]);
        assert_eq!(res.len(), 2);
    }
}
