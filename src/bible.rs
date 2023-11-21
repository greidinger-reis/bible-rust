#![allow(dead_code)]
use quick_xml::de::{from_reader, DeError};
use rand::{seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Default, Deserialize)]
#[serde(rename = "bible")]
pub struct Bible {
    #[serde(rename = "book")]
    pub books: Vec<BibleBook>,
}

#[derive(Debug, Default, Deserialize)]
pub struct BibleBook {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@abbrev")]
    pub abbrev: String,
    #[serde(rename = "@chapters")]
    pub chapters_len: usize,

    #[serde(rename = "c")]
    pub chapters: Vec<BibleBookChapter>,
}

#[derive(Debug, Default, Deserialize)]
pub struct BibleBookChapter {
    #[serde(rename = "@n")]
    pub number: usize,

    #[serde(rename = "v")]
    pub verses: Vec<BibleBookVerse>,
}

#[derive(Debug, Default, Deserialize)]
pub struct BibleBookVerse {
    #[serde(rename = "@n")]
    pub number: usize,
    #[serde(rename = "$text")]
    pub content: String,
}

pub enum RandomVerseOpts {
    All,
    OldTestamentOnly,
    NewTestamentOnly,
}

pub enum VerseOpts {
    Single(usize),
    Range(usize, usize),
}

#[derive(Debug, Serialize)]
pub enum BibleVerseResult {
    Single(BibleSingleVerseResult),
    Range(BibleRangeVerseResult),
}

pub struct Abbreviation {
    pub book: String,
    pub chapter: usize,
    pub verse: VerseOpts,
}

#[derive(Debug, Serialize)]
pub struct BibleSingleVerseResult {
    pub book: String,
    pub chapter: usize,
    pub verse: usize,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct VerseRange {
    pub number: usize,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct BibleRangeVerseResult {
    pub book: String,
    pub chapter: usize,
    pub verses: Vec<VerseRange>,
}

impl FromStr for VerseOpts {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();

        if parts.len() > 2 {
            return Err("Invalid verse format");
        }

        let start: usize = parts[0].parse().map_err(|_| "Invalid start verse number")?;

        match parts.len() {
            1 => Ok(VerseOpts::Single(start)),
            2 => {
                let end: usize = parts[1].parse().map_err(|_| "Invalid end verse number")?;
                Ok(VerseOpts::Range(start, end))
            }
            _ => Err("Invalid verse format"),
        }
    }
}

impl FromStr for Abbreviation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Splitting the input string into book, chapter, and verse parts
        let parts: Vec<&str> = s.split(':').collect();

        if parts.len() != 3 {
            eprintln!("{:?}", parts);
            return Err("Invalid format. Must be in the form 'book:chapter:verse'");
        }

        let book = parts[0];

        // Parsing chapter
        let chapter: usize = parts[1].parse().map_err(|_| "Invalid chapter number")?;

        // Splitting the book and verse parts
        let verses_opts: VerseOpts = parts[2].parse()?;

        Ok(Abbreviation {
            book: book.to_string(),
            chapter,
            verse: verses_opts,
        })
    }
}

impl Bible {
    pub fn from_xml_file(path: &str) -> Result<Bible, DeError> {
        let reader = quick_xml::Reader::from_file(path)?;
        let bible: Bible = from_reader(reader.into_inner())?;

        Ok(bible)
    }

    pub fn random(&self, opts: RandomVerseOpts, count: usize) -> BibleVerseResult {
        let books = match opts {
            RandomVerseOpts::All => &self.books,
            RandomVerseOpts::OldTestamentOnly => &self.books[0..39],
            RandomVerseOpts::NewTestamentOnly => &self.books[39..],
        };

        let book = books.choose(&mut rand::thread_rng()).unwrap();
        let chapter = book.chapters.choose(&mut rand::thread_rng()).unwrap();

        let mut rng = rand::thread_rng();

        let start_index = rng.gen_range(0..chapter.verses.len());
        let mut stop_index = start_index + count;

        if stop_index > chapter.verses.len() {
            stop_index = chapter.verses.len()
        }

        let verses = &chapter.verses[start_index..stop_index];

        if verses.len() > 1 {
            return BibleVerseResult::Range(BibleRangeVerseResult {
                book: book.name.clone(),
                chapter: chapter.number,
                verses: verses
                    .iter()
                    .map(|v| {
                        return VerseRange {
                            number: v.number,
                            content: v.content.clone(),
                        };
                    })
                    .collect(),
            });
        }

        BibleVerseResult::Single(BibleSingleVerseResult {
            book: book.name.clone(),
            chapter: chapter.number,
            verse: verses.first().unwrap().number,
            content: verses.first().unwrap().content.clone(),
        })
    }

    pub fn get(
        &self,
        book_name: &str,
        chapter: usize,
        verse: VerseOpts,
    ) -> Option<BibleVerseResult> {
        let book = self.books.iter().find(|b| b.name == book_name)?;
        let chapter = book.chapters.iter().find(|c| c.number == chapter)?;

        let verses: Vec<&BibleBookVerse> = match verse {
            VerseOpts::Single(verse) => chapter
                .verses
                .iter()
                .filter(|v| v.number == verse)
                .collect(),
            VerseOpts::Range(verse_start, verse_final) => chapter
                .verses
                .iter()
                .filter(|v| v.number >= verse_start && v.number <= verse_final)
                .collect(),
        };

        if verses.len() > 1 {
            Some(BibleVerseResult::Range(BibleRangeVerseResult {
                book: book.name.clone(),
                chapter: chapter.number,
                verses: verses
                    .iter()
                    .map(|v| VerseRange {
                        number: v.number,
                        content: v.content.clone(),
                    })
                    .collect(),
            }))
        } else {
            let verse = verses.first().unwrap();
            Some(BibleVerseResult::Single(BibleSingleVerseResult {
                book: book.name.clone(),
                chapter: chapter.number,
                verse: verse.number,
                content: verse.content.clone(),
            }))
        }
    }

    pub fn get_abbr(&self, abbrev: Abbreviation) -> Option<BibleVerseResult> {
        let book = self.books.iter().find(|b| b.abbrev == abbrev.book)?;
        let chapter = book.chapters.iter().find(|c| c.number == abbrev.chapter)?;
        let verses: Vec<&BibleBookVerse> = match abbrev.verse {
            VerseOpts::Range(start, end) => chapter
                .verses
                .iter()
                .filter(|v| v.number >= start && v.number <= end)
                .collect(),
            VerseOpts::Single(verse) => chapter
                .verses
                .iter()
                .filter(|v| v.number == verse)
                .collect(),
        };

        if verses.len() > 1 {
            Some(BibleVerseResult::Range(BibleRangeVerseResult {
                book: book.name.clone(),
                chapter: chapter.number,
                verses: verses
                    .into_iter()
                    .map(|v| VerseRange {
                        number: v.number,
                        content: v.content.clone(),
                    })
                    .collect(),
            }))
        } else {
            let verse = verses.first().unwrap();

            Some(BibleVerseResult::Single(BibleSingleVerseResult {
                book: book.name.clone(),
                chapter: chapter.number,
                verse: verse.number,
                content: verse.content.clone(),
            }))
        }
    }
}
