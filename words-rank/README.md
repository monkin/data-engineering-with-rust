# Calculate word rank for specified text

This is a CLI program written in Rust that calculates the rank of words in a provided text.
First it creates a graph of words with their connections, then it calculates the rank of each word using pagerank algorithm.

```
Usage: words-rank.exe [OPTIONS]

Options:
  -f, --file <FILE_NAME>                           Path to the file to be processed
  -u, --url <URL>
  -w, --min-word-length <MIN_WORD_LENGTH>          [default: 4]
  -s, --min-sentence-length <MIN_SENTENCE_LENGTH>  [default: 3]
  -w, --words-count <WORDS_COUNT>                  [default: 10]
  -h, --help                                       Print help
  -V, --version                                    Print version
```

Yuu can test it with an url or a file. For example:

```sh
cargo run --release -- --url https://gist.githubusercontent.com/phillipj/4944029/raw/75ba2243dd5ec2875f629bf5d79f6c1e4b5a8b46/alice_in_wonderland.txt --min-word-length 5 --words-count 30
```

It prints a table with the top 30 words with 5 or more characters from the provided text.

````
┌────┬─────────┬──────────────┐
│ #  │ word    │ rank         │
├────┼─────────┼──────────────┤
│ 0  │ alice   │ 0.022153288  │
│ 1  │ little  │ 0.011358092  │
│ 2  │ about   │ 0.0093586035 │
│ 3  │ herself │ 0.008037095  │
│ 4  │ would   │ 0.0076274537 │
│ 5  │ there   │ 0.0070018717 │
│ 6  │ could   │ 0.0068882247 │
│ 7  │ queen   │ 0.0066170106 │
│ 8  │ again   │ 0.0065935245 │
│ 9  │ began   │ 0.005635563  │
│ 10 │ quite   │ 0.0055079693 │
│ 11 │ their   │ 0.004988727  │
│ 12 │ which   │ 0.0047958773 │
│ 13 │ turtle  │ 0.0047576516 │
│ 14 │ first   │ 0.0046323338 │
│ 15 │ other   │ 0.004629598  │
│ 16 │ voice   │ 0.004488325  │
│ 17 │ hatter  │ 0.004474386  │
│ 18 │ thought │ 0.0044569597 │
│ 19 │ round   │ 0.0043708724 │
│ 20 │ after   │ 0.0043251226 │
│ 21 │ great   │ 0.0042741527 │
│ 22 │ gryphon │ 0.0041979924 │
│ 23 │ before  │ 0.0040422482 │
│ 24 │ never   │ 0.0040382887 │
│ 25 │ thing   │ 0.0039726617 │
│ 26 │ large   │ 0.0038968688 │
│ 27 │ rabbit  │ 0.0038865886 │
│ 28 │ think   │ 0.003785115  │
│ 29 │ looked  │ 0.0037749696 │
└────┴─────────┴──────────────┘
```