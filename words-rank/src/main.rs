use crate::words_graph::WordsGraph;
use clap::Parser;
use error::Error;
use std::fs::read_to_string;

mod error;
mod words_graph;

#[derive(Parser, Debug)]
#[clap(
    version = "1.0",
    author = "Andrei Monkin",
    about = "CLI program that computes words rank in the provided text"
)]
struct CliArguments {
    /// Path to the file to be processed
    #[clap(short, long = "file")]
    file_name: Option<String>,
    #[clap(short, long)]
    url: Option<String>,
    // Minimum word length in characters
    #[clap(short = 'w', long, default_value_t = 4usize)]
    min_word_length: usize,
    // Minimum sentence length in words
    #[clap(short = 's', long, default_value_t = 3usize)]
    min_sentence_length: usize,
    #[clap(short, long, default_value_t = 10usize)]
    words_count: usize,
}

impl CliArguments {
    fn load_content(&self) -> Result<String, Error> {
        if (self.file_name.is_none() && self.url.is_none())
            || (self.file_name.is_some() && self.url.is_some())
        {
            Err(Error::new(
                "One of the parameters '--url' or '--file' must be provided",
            ))
        } else if let Some(file) = &self.file_name {
            return read_to_string(&file).map_err(|error| {
                Error::new(&format!(
                    "Could not read file: \"{}\"; error: {}",
                    file, error
                ))
            });
        } else if let Some(url) = &self.url {
            let response = reqwest::blocking::get(url)?;
            if !response.status().is_success() {
                return Err(Error::new(&format!(
                    "Could not fetch URL: \"{}\"; status: {}",
                    url,
                    response.status()
                )));
            }
            return Ok(response.text()?);
        } else {
            Err(Error::new("Unreachable code"))
        }
    }
}

fn main() {
    let args = CliArguments::parse();
    let content = args
        .load_content()
        .unwrap_or_else(|error| panic!("{}", &error.message));
    let mut bump = bumpalo::Bump::new();
    let graph = WordsGraph::parse(
        &mut bump,
        &content,
        args.min_word_length,
        args.min_sentence_length,
    );
    let mut table = ascii_table::AsciiTable::default();
    table.set_max_width(80);
    table.column(0).set_header("#");
    table.column(1).set_header("word");
    table.column(2).set_header("rank");

    let data: Vec<_> = graph
        .get_important_words(args.words_count)
        .iter()
        .enumerate()
        .map(|(i, (word, rank))| vec![i.to_string(), word.to_string(), rank.to_string()])
        .collect();

    table.print(&data);
}
