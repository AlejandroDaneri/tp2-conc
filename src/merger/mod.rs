pub struct Merger {}

impl Merger {
    pub fn merge(first: &Vec<String>, second: &Vec<String>) -> Vec<String> {
        [&first[..], &second[..]].concat()
    }
}

#[cfg(test)]
mod test_merge {
    #[test]
    fn internal() {
        use crate::merger::Merger;
        let mut expected = Vec::<String>::new();
        expected.push("as".to_string());
        expected.push("bs".to_string());
        expected.push("cs".to_string());
        expected.push("ds".to_string());

        let mut s1 = Vec::<String>::new();
        s1.push("as".to_string());
        s1.push("bs".to_string());

        let mut s2 = Vec::<String>::new();
        s2.push("cs".to_string());
        s2.push("ds".to_string());
        let res = Merger::merge(&s1, &s2);

        assert_eq!(expected, res);
    }
}
