use bumpalo::Bump;
use petgraph::algo::page_rank::page_rank;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;
use std::cmp::Ordering;
use std::collections::HashMap;

const PAGE_RANK_DAMPING_FACTOR: f32 = 0.85;
const PAGE_RANK_MAX_ITERATIONS: usize = 10;

pub struct WordsGraph<'a> {
    graph: Graph<&'a str, u32, Undirected>,
    indexes: HashMap<&'a str, NodeIndex>,
}

impl<'a> WordsGraph<'a> {
    fn add_node(&mut self, word: &'a str) -> NodeIndex {
        if let Some(index) = self.indexes.get(word) {
            *index
        } else {
            let index = self.graph.add_node(word);
            self.indexes.insert(word, index);
            index
        }
    }

    fn add_edge(&mut self, word1: &'a str, word2: &'a str) {
        if word1 != word2 {
            let i1 = self.add_node(word1);
            let i2 = self.add_node(word2);
            self.graph.update_edge(i1, i2, 1);
        }
    }

    pub fn parse(
        bump: &'a Bump,
        input: &str,
        min_word_length: usize,
        min_sentence_length: usize,
    ) -> Self {
        let mut words_graph = Self {
            graph: Graph::new_undirected(),
            indexes: HashMap::new(),
        };

        let content = input.to_lowercase();
        let sentences = content
            .split(|char| ".!?".contains(char))
            .map(|sentence| {
                sentence
                    .split(|char| !char::is_alphabetic(char))
                    .filter(|word| word.len() >= min_word_length)
                    .map(|word| &*bump.alloc_str(word))
                    .collect::<Vec<_>>()
            })
            .filter(|sentence| sentence.len() >= min_sentence_length);

        for sentence in sentences {
            for words in sentence.windows(2) {
                words_graph.add_edge(words[0], words[1]);
            }
        }

        words_graph
    }

    /// Collects the important words from the graph using page rank algorithm
    pub fn get_important_words(&self, words_count: usize) -> Vec<(&'a str, f32)> {
        let ranks = page_rank(
            &self.graph,
            PAGE_RANK_DAMPING_FACTOR,
            PAGE_RANK_MAX_ITERATIONS,
        );
        let mut ranks: Vec<_> = ranks.iter().copied().enumerate().collect();
        ranks.sort_by(|(_, rank1), (_, rank2)| rank2.partial_cmp(rank1).unwrap_or(Ordering::Equal));
        ranks
            .iter()
            .take(words_count)
            .map(|(index, rank)| {
                (
                    *self.graph.node_weight(NodeIndex::new(*index)).unwrap(),
                    *rank,
                )
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bumpalo::Bump;

    #[test]
    fn test_parse() {
        let bump = Bump::new();
        let input = "Hello, world! This is a test sentence. Another test sentence.";
        let min_word_length = 3;
        let min_sentence_length = 2;
        let words_graph = WordsGraph::parse(&bump, input, min_word_length, min_sentence_length);

        assert_eq!(words_graph.graph.node_count(), 6);
        assert_eq!(words_graph.graph.edge_count(), 4);
    }

    #[test]
    fn test_get_important_words() {
        let bump = Bump::new();
        let input = "Hello, world! This is a test sentence. Another test sentence.";
        let min_word_length = 3;
        let min_sentence_length = 2;
        let words_graph = WordsGraph::parse(&bump, input, min_word_length, min_sentence_length);

        let important_words = words_graph.get_important_words(3);

        assert_eq!(important_words.len(), 3);
        assert_eq!(
            important_words,
            vec![
                ("test", 0.28293315),
                ("hello", 0.18177393),
                ("world", 0.18177393)
            ]
        );
    }
}
