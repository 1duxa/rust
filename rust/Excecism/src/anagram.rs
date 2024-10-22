use std::collections::HashSet;
#[allow(dead_code)]
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&'a str]) -> HashSet<&'a str> {
    let mut word_chars: Vec<char> = word.to_lowercase().chars().collect();
    word_chars.sort_unstable();

    let mut res: HashSet<&'a str> = HashSet::new();

    for &anagram in possible_anagrams {
        let mut anagram_chars: Vec<char> = anagram.to_lowercase().chars().collect();
        anagram_chars.sort_unstable();

        if word_chars == anagram_chars && word.to_lowercase() != anagram.to_lowercase() {
            res.insert(anagram);
        }
    }

    res
}
