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
        println!(
            "{:?}",
            serde_json::to_string(&result_random.unwrap()).unwrap()
        );
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
        println!(
            "{:?}",
            serde_json::to_string(&result_abbrev.unwrap()).unwrap()
        );
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
    println!("{:?}", serde_json::to_string(&result).unwrap());
    std::process::exit(0);
}
