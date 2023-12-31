mod args;
mod bible;
mod utils;

use args::{RustBibleArgs, SubCommands};
use bible::{Bible, BibleVerseResult, RandomVerseOpts, VerseOpts};
use clap::Parser;
use utils::SubscriptRepresentation;

use crate::bible::Abbreviation;

fn main() {
    let args = RustBibleArgs::parse();

    let bible = Bible::from_xml_file(&args.file_path)
        .map_err(|e| {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        })
        .unwrap();

    let result_random: Option<BibleVerseResult> = match args.sub_command {
        None => None,
        Some(sub_commands) => {
            match sub_commands {
                SubCommands::Random(r_args) => {
                    if r_args.new_testment_only && r_args.old_testment_only {
                        eprintln!("Error: Cannot specify both --new-testment-only and --old-testment-only");
                        std::process::exit(1);
                    }

                    let verse_count = r_args.verse_count.unwrap_or(1);

                    if r_args.new_testment_only {
                        Some(bible.random(RandomVerseOpts::NewTestamentOnly, verse_count))
                    } else if r_args.old_testment_only {
                        Some(bible.random(RandomVerseOpts::OldTestamentOnly, verse_count))
                    } else {
                        Some(bible.random(RandomVerseOpts::All, verse_count))
                    }
                }
            }
        }
    };

    if let Some(result) = result_random {
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
                for v in verses {
                    let verse_number = v.number.to_subscript();
                    let verse_content = v.content;
                    println!("{verse_number} {verse_content}");
                }
            }
        }

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

                println!("{book} {chapter}");

                for verse in verses {
                    let verse_content = verse.content;
                    let verse_number = verse.number.to_subscript();

                    println!("{verse_number} {verse_content}")
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
