use itertools::Itertools;
use std::collections::HashSet;
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let normalized_word = normalize_string(word);
    possible_anagrams
        .iter()
        .filter(|item| {
            normalize_string(item) == normalized_word && item.to_lowercase() != word.to_lowercase()
        })
        .map(|item| *item)
        .collect()
}
fn normalize_string(input: &str) -> String {
    input.to_lowercase().chars().sorted().collect()
}
