//! 2785. Sort Vowels in a String

use std::collections::{BTreeMap, HashSet, VecDeque};

pub struct Solution;

impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let vowel_set = HashSet::from(['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U']);
        let mut vowel_map = BTreeMap::new();
        let mut index_vec = VecDeque::new();
        let mut res = vec![];

        for (index, char) in s.chars().enumerate() {
            if vowel_set.contains(&char) {
                let count = *vowel_map.entry(char).or_insert(0) + 1;
                vowel_map.insert(char, count);
                index_vec.push_back(index);
            }
            res.push(char);
        }

        for (char, count) in vowel_map {
            for _ in 0..count {
                let index = index_vec.pop_front().unwrap();
                res[index] = char;
            }
        }

        res.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::sort_vowels("lYmpH".to_string()),
            "lYmpH".to_string()
        );
        assert_eq!(
            Solution::sort_vowels("aeiou".to_string()),
            "aeiou".to_string()
        );
        assert_eq!(
            Solution::sort_vowels("AaEeIiOoUu".to_string()),
            "AEIOUaeiou".to_string()
        );
        assert_eq!(
            Solution::sort_vowels("lEetcOde".to_string()),
            "lEOtcede".to_string()
        );
    }
}