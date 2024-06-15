use clap::Parser;
use std::collections::HashMap;

/// Returns a map of words and their counts in the input string
fn get_words_count_map(input: &str) -> HashMap<&str, u32> {
    input
        .split(|char| !('a'..='z').contains(&char) && !('A'..='Z').contains(&char))
        .filter(|word| !word.is_empty())
        .fold(HashMap::new(), |mut map, word| {
            *map.entry(word).or_insert(0) += 1;
            map
        })
}

#[derive(Parser, Debug)]
#[clap(
    version = "1.0",
    author = "Andrei Monkin",
    about = "CLI program that counts words in a text file"
)]
struct CliArguments {
    /// Path to the file to be processed
    #[clap(short, long = "file", required = true)]
    file_name: String,
}

fn main() {
    let arguments = CliArguments::parse();
    let file_name = arguments.file_name;

    match std::fs::read_to_string(&file_name) {
        Ok(content) => {
            let count_map = get_words_count_map(&content);
            for (word, count) in count_map {
                println!("{}: {}", word, count);
            }
            std::process::exit(0);
        }
        Err(error) => {
            eprintln!("Error: {}; Could not read file: \"{}\"", error, file_name);
            std::process::exit(1);
        }
    }
}
