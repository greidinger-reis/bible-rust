#![allow(dead_code, unused_variables)]

use quick_xml::de::{from_str, DeError};
use serde::Deserialize;

const FILE_STR: &'static str = include_str!("../bible-por-nvi.xml");

#[derive(Debug, Default, Deserialize)]
#[serde(rename = "bible")]
struct Bible {
    #[serde(rename = "book")]
    books: Vec<BibleBook>,
}

impl Bible {
    fn from_xml() -> Result<Bible, DeError> {
        let bible: Bible = from_str(FILE_STR)?;

        Ok(bible)
    }
}

#[derive(Debug, Default, Deserialize)]
struct BibleBook {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@abbrev")]
    abbrev: String,
    #[serde(rename = "@chapters")]
    chapters_len: String,

    #[serde(rename = "c")]
    chapters: Vec<BibleBookChapter>,
}

#[derive(Debug, Default, Deserialize)]
struct BibleBookChapter {
    #[serde(rename = "@n")]
    number: String,

    #[serde(rename = "v")]
    verses: Vec<BibleBookVerse>,
}

#[derive(Debug, Default, Deserialize)]
struct BibleBookVerse {
    #[serde(rename = "@n")]
    number: String,
    #[serde(rename = "$text")]
    content: String,
}


fn main() {
    let bible = Bible::from_xml().unwrap();
    println!("Bible: {:?}", bible);

    // println!("{:?}", bible.books[0].chapters[0].verses[0].content);
}
