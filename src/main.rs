mod args;
mod bible;

use args::{RustBibleArgs, SubCommands};
use bible::{Bible, BibleVerseResult, RandomVerseOpts, VerseOpts};
use clap::Parser;

use crate::bible::{Abbreviation, BibleSingleVerseResult};

fn main() {
    let args = RustBibleArgs::parse();

    let bible = Bible::from_xml_file(&args.file_path)
        .map_err(|e| {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        })
        .unwrap();

    let result_random: Option<BibleSingleVerseResult> = match args.sub_command {
        None => None,
        Some(sub_commands) => {
            match sub_commands {
                SubCommands::Random(r_args) => {
                    if r_args.new_testment_only && r_args.old_testment_only {
                        eprintln!("Error: Cannot specify both --new-testment-only and --old-testment-only");
                        std::process::exit(1);
                    }

                    if r_args.new_testment_only {
                        Some(bible.random(RandomVerseOpts::NewTestamentOnly))
                    } else if r_args.old_testment_only {
                        Some(bible.random(RandomVerseOpts::OldTestamentOnly))
                    } else {
                        Some(bible.random(RandomVerseOpts::All))
                    }
                }
            }
        }
    };

    if result_random.is_some() {
        let res = result_random.unwrap();
        let book = res.book;
        let chapter = res.chapter;
        let verse = res.verse;
        let content = res.content;

        println!("{book} {chapter}:{verse}\n{content}");

        std::process::exit(0);
    }

    let result_abbrev: Option<BibleVerseResult> = match args.abbreviation {
        None => None,
        Some(abbrev) => {
            let abbrev_obj: Abbreviation = abbrev.parse().expect("Invalid Abbreviation value");
            bible.get_abbr(abbrev_obj)
        }
    };

    if result_abbrev.is_some() {
        match result_abbrev.unwrap() {
            BibleVerseResult::Range(res) => {
                let book = res.book;
                let chapter = res.chapter;
                let verses = res.verses;
                let verse_start_n = verses.first().unwrap().number;
                let verse_end_n = verses.last().unwrap().number;

                println!("{book} {chapter}:{verse_start_n}-{verse_end_n}");

                for verse in verses {
                    let verse_content = verse.content;

                    println!("{verse_content}")
                }
            }
            BibleVerseResult::Single(res) => {
                let book = res.book;
                let chapter = res.chapter;
                let verse = res.verse;
                let content = res.content;

                println!("{book} {chapter}:{verse}\n{content}");
            }
        }
        std::process::exit(0);
    }

    let book_name = args.book.expect("Book name is required");
    let chapter_number = args.chapter.expect("Chapter number is required");
    let verse_opts: VerseOpts = args
        .verses
        .expect("Verses number is required")
        .parse()
        .unwrap();

    let result = bible
        .get(&book_name, chapter_number, verse_opts)
        .expect("Verse not found");

    match result {
        BibleVerseResult::Single(res) => {
            let book = res.book;
            let chapter = res.chapter;
            let verse = res.verse;
            let content = res.content;

            println!("{book} {chapter}:{verse}\n{content}");
        }
        BibleVerseResult::Range(res) => {
            let book = res.book;
            let chapter = res.chapter;
            let verses = res.verses;
            println!("{book} {chapter}");

            for verse in verses {
                let verse_number = verse.number;
                let verse_content = verse.content;

                println!("{verse_number}: {verse_content}");
            }
        }
    };
    std::process::exit(0);
}
