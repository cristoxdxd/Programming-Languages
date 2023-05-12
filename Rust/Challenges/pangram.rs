use std::collections::HashSet;

fn is_pangram(sentence: &str) -> bool {
    let mut letters = HashSet::new();
    for c in sentence.chars() {
        if c.is_alphabetic() {
            letters.insert(c.to_ascii_lowercase());
        }
    }
    letters.len() == 26
}

fn main() {
    let sentence = "The quick brown fox jumps over the lazy dog";
    println!("Is \"{}\" a pangram? {}", sentence, is_pangram(sentence));
}