use itertools;
use itertools::Itertools;
use std::collections::HashSet;
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let normalized_word = normalize_string(word);
    let mut set: HashSet<&str> = HashSet::new();
    let valid_items: Vec<&str> = possible_anagrams
        .iter()
        .filter(|item| {
            normalize_string(item).eq_ignore_ascii_case(&normalized_word)
                && item.to_lowercase() != word.to_lowercase()
        })
        .map(|item| *item)
        .collect();
    for anagram in valid_items {
        set.insert(anagram);
    }
    set
}
fn normalize_string(input: &str) -> String {
    input.to_lowercase().chars().sorted().collect()
}
