extern crate reqwest;

use reqwest::blocking;
use std::env::args;

const ARGS_MIN_LEN: usize = 2;

fn extract_thesaurus(text: &str) {
    let mut search_l = 0;
    let mut search_r = 0;

    match text.find("Compare Synonyms") {
        None => {}
        Some(index) => search_l = index,
    };

    let (_left, right) = text.split_at(search_l);

    match right.rfind("<span color=\"#1A1A1A\" font-size=\"16px\"") {
        None => {}
        Some(index) => search_r = index,
    };

    let (left2, _right2) = right.split_at(search_r);
    let mut vec: Vec<&str> = left2.split("href=\"/browse/").collect();
    vec.remove(0);

    let mut res: Vec<&str> = Vec::new();

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

        res.push(left2)
    }

    println!("{:?}", res);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let argv: Vec<String> = args().collect();
    if args().len() < ARGS_MIN_LEN {
        // println!("Usage: {:?} <input file>", argv[0]);
        println!("Usage: specify word to search as an argument");
    } else {
        let request = blocking::get(format!("https://www.thesaurus.com/browse/{}", argv[1]))?;
        let text = request.text()?;
        let string = text.as_str();
        extract_thesaurus(string);
    }
    Ok(())
}
