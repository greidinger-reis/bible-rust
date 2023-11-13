# Rust Bible CLI App

A command-line interface (CLI) application written in Rust for reading Bible verses from an XML file. The app utilizes the `clap` crate for parsing command-line arguments.

## Installation

```bash
git clone https://github.com/yourusername/rust-bible-cli.git
cd rust-bible-cli
cargo build --release
```

The compiled binary will be available in the `target/release` directory.

## Usage

### The file
You will need to download a XML file in this format
```xml
<bible>
  <book name="" abbrev="" chapters="">
    <c n="">
      <v n="">Verse Text</v>
    </c>
  </book>
</bible>
```
🇧🇷 I'm using one downloaded from this source 
https://github.com/thiagobodruk/biblia/tree/master/xml

Run the script
```bash
rust-bible-cli [OPTIONS] --file-path <file_path> [--abbreviation <abbreviation>] [--book <book>] [--chapter <chapter>] [--verses <verses>] [SUBCOMMAND]
```

### Options:

- `--file-path`, `-f`: The file path to the XML file containing the Bible.

- `--abbreviation`, `-a`: A string containing the abbreviation of the book name and the chapter, verse number separated by a colon. A range of verses can be specified. Examples: "jn:3:16", "1co:13:4", "jn:3:16-18".

- `--book`, `-b`: The book name. Examples: "John", "1 Corinthians". The name should be in the same language as the Bible file.

- `--chapter`, `-c`: The chapter number.

- `--verses`, `-v`: The verse number or range of verses. Examples: "16", "4", "16-18".

### Subcommands:

#### Random

```bash
rust-bible-cli random [OPTIONS]
```

##### Options:

- `--new-testament-only`, `-nt`: Get only verses from the New Testament.

- `--old-testament-only`, `-ot`: Get only verses from the Old Testament.

### Examples:

1. Get a specific verse:

```bash
rust-bible-cli --file-path path/to/bible.xml --abbreviation jn:3:16
```

2. Get verses from a specific book, chapter, and range of verses:

```bash
rust-bible-cli --file-path path/to/bible.xml --book "John" --chapter 3 --verses 16-18
```

3. Get a random verse from the New Testament:

```bash
rust-bible-cli random --file-path path/to/bible.xml --new-testament-only
```

4. Get a random verse from the Old Testament:

```bash
rust-bible-cli random --file-path path/to/bible.xml --old-testament-only
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
