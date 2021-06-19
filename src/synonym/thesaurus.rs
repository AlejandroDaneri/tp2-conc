use super::Finder;

pub struct Thesaurus {
    word: String,
}

impl Thesaurus {
    pub fn new_query(word: &str) -> Self {
        Self {
            word: word.to_owned(),
        }
    }
}

impl Finder for Thesaurus {
    fn url(&self) -> String {
        format!("https://www.thesaurus.com/browse/{}", self.word.as_str())
    }

    fn parse_body(&self, body: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut search_l = 0;
        let mut search_r = 0;

        match body.find("Compare Synonyms") {
            None => {}
            Some(index) => search_l = index,
        };

        let (_left, right) = body.split_at(search_l);

        match right.rfind("<span color=\"#1A1A1A\" font-size=\"16px\"") {
            None => {}
            Some(index) => search_r = index,
        };

        let (left2, _right2) = right.split_at(search_r);
        let mut vec: Vec<&str> = left2.split("href=\"/browse/").collect();
        vec.remove(0);

        let mut res: Vec<String> = Vec::new();

        for elem in vec.iter() {
            let mut search_l = 0;
            let mut search_r = 0;

            match elem.find('>') {
                None => {}
                Some(index) => search_l = index + 1,
            };

            let (_left, right) = elem.split_at(search_l);
            match right.find('<') {
                None => {}
                Some(index) => search_r = index,
            };
            let (left2, _right2) = right.split_at(search_r);
            res.push(left2.to_owned())
        }

        Ok(res)
    }
}
