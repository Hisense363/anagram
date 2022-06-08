use std::collections::{HashSet, HashMap};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let check = word.to_string().to_lowercase();
    let mut word_map: HashMap<String, i32> = HashMap::new();
    let mut lib: HashSet<&str> = HashSet::new();
    for val in check.chars() {
        *word_map.entry(val.to_string()).or_insert(0) += 1;
    }
    for val in possible_anagrams {
        let lowercased = &val.to_lowercase();
        if lowercased.len() == check.len() && &check != lowercased && is_anagram(&word_map, lowercased) {
            lib.insert(val);
        }
    }
    return lib;
}

pub fn is_anagram (original: &HashMap<String, i32>, word_vec: &String) -> bool {
    let mut new_original = original.clone();
    for val in word_vec.chars() {
        if new_original.contains_key(&val.to_string()) && new_original[&val.to_string()] > 0 {
            *new_original.get_mut(&val.to_string()).unwrap() -= 1;
        }else {
            return false;
        }
    }
    return true;
}