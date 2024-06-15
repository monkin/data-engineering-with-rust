use std::collections::HashMap;

/// Returns a map of words and their counts in the input string
pub fn get_words_count_map(input: &str) -> HashMap<&str, u32> {
    input
        .split(|char| !('a'..='z').contains(&char) && !('A'..='Z').contains(&char))
        .filter(|word| !word.is_empty())
        .fold(HashMap::new(), |mut map, word| {
            *map.entry(word).or_insert(0) += 1;
            map
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_words_count_map() {
        let input = "Hello, world! Hello, Rust!";
        let expected: HashMap<&str, u32> = vec![("Hello", 2), ("world", 1), ("Rust", 1)]
            .into_iter()
            .collect();
        assert_eq!(get_words_count_map(input), expected);
    }
}
