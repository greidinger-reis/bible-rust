#![allow(dead_code, unused_variables)]

use quick_xml::de::{from_str, DeError};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

const FILE_STR: &'static str = include_str!("../bible-por-nvi.xml");

#[derive(Debug, Default, Deserialize )]
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

    fn random(&self) -> BibleVerseResponse {
        let book = self.books.choose(&mut rand::thread_rng()).unwrap();
        let chapter = book.chapters.choose(&mut rand::thread_rng()).unwrap();
        let verse = chapter.verses.choose(&mut rand::thread_rng()).unwrap();

        BibleVerseResponse {
            book: book.name.clone(),
            chapter: chapter.number,
            verse: verse.number,
            content: verse.content.clone(),
        }
    }

    fn get(&self, book_name: &str, chapter: usize, verse: usize) -> Option<BibleVerseResponse> {
        let book = self.books.iter().find(|b| b.name == book_name)?;
        let chapter = book.chapters.iter().find(|c| c.number == chapter)?;
        let verse = chapter.verses.iter().find(|v| v.number == verse)?;

        Some(BibleVerseResponse {
            book: book.name.clone(),
            chapter: chapter.number,
            verse: verse.number,
            content: verse.content.clone(),
        })
    }

    fn get_by_abbr(
        &self,
        book_abbr: &str,
        chapter: usize,
        verse: usize,
    ) -> Option<BibleVerseResponse> {
        let book = self.books.iter().find(|b| b.abbrev == book_abbr)?;
        let chapter = book.chapters.iter().find(|c| c.number == chapter)?;
        let verse = chapter.verses.iter().find(|v| v.number == verse)?;

        Some(BibleVerseResponse {
            book: book.name.clone(),
            chapter: chapter.number,
            verse: verse.number,
            content: verse.content.clone(),
        })
    }
}

#[derive(Debug, Serialize)]
struct BibleVerseResponse {
    book: String,
    chapter: usize,
    verse: usize,
    content: String,
}

#[derive(Debug, Default, Deserialize)]
struct BibleBook {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@abbrev")]
    abbrev: String,
    #[serde(rename = "@chapters")]
    chapters_len: usize,

    #[serde(rename = "c")]
    chapters: Vec<BibleBookChapter>,
}

#[derive(Debug, Default, Deserialize)]
struct BibleBookChapter {
    #[serde(rename = "@n")]
    number: usize,

    #[serde(rename = "v")]
    verses: Vec<BibleBookVerse>,
}

#[derive(Debug, Default, Deserialize)]
struct BibleBookVerse {
    #[serde(rename = "@n")]
    number: usize,
    #[serde(rename = "$text")]
    content: String,
}

fn main() {
    let bible = Bible::from_xml().unwrap();
    let rand_verse = bible.random();
    let verse = bible.get("Gênesis", 1, 1).unwrap();
    let verse_abbr = bible.get_by_abbr("gn", 1, 1).unwrap();
    println!("Random Verse: {:?}", rand_verse);
    println!("Verse: {:?}", verse);
    println!("Verse by Abbr: {:?}", verse_abbr);
}
