use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct RustBibleArgs {
    /// The file path to the XML file containing the Bible
    #[arg(long, short)]
    pub file_path: String,

    /// A string containing the abbreviation of the book name and the chapter, verse number separated by a colon. A range of verses can be specified. Examples: "jn:3:16", "1co:13:4", "jn:3:16-18"
    #[arg(long, short)]
    pub abbreviation: Option<String>,

    /// The book name. Examples: "John", "1 Corinthians", "1 Corinthians" The name should be in the
    /// same language as the Bible file.
    #[arg(long, short)]
    pub book: Option<String>,

    /// The Chapter number
    #[arg(long, short)]
    pub chapter: Option<usize>,

    /// The verse number or range of verses. Examples: "16", "4", "16-18"
    #[arg(long, short)]
    pub verses: Option<String>,

    #[clap(subcommand)]
    pub sub_command: Option<SubCommands>,
}

#[derive(Subcommand, Debug)]
pub enum SubCommands {
    /// Get a random verse
    Random(RandomArgs),
}

#[derive(Args, Debug)]
pub struct RandomArgs {
    /// Get only verses from the New Testament
    #[arg(long, short, action)]
    pub new_testment_only: bool,
    /// Get only verses from the Old Testament
    #[arg(long, short, action)]
    pub old_testment_only: bool,

    /// Amount of verses to get
    #[arg(long, short)]
    pub verse_count: Option<usize>,
}
