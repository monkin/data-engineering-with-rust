/*
 * Usage: word-count --file <FILE_NAME>
 *
 * Options:
 *     -f, --file <FILE_NAME>  Path to the file to be processed
 *     -h, --help              Print help
 *     -V, --version           Print version
*/
use clap::Parser;

mod words_count;

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
            let count_map = words_count::get_words_count_map(&content);
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
