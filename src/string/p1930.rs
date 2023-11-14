//! 1930. Unique Length-3 Palindromic Subsequences

use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {

    // We need to count the number of unique palindromic subsequences of length 3 that have
    // the same letters in the first and last position.
    // This will help us solve the problem by counting the letters in between.
    //
    // The palindromic subsequence in the string "abcda" can only start and end with the letter 'a'.
    // Therefore, the count of unique palindromic subsequences is equal to the number of
    // unique letters in the string "bcd", which is 3.
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        if s.len() < 3 {
            return 0;
        }

        // Find the first and last index of each character appear in the string.
        let mut map = HashMap::new();
        for (index, char) in s.chars().enumerate() {
            let pair = map.entry(char).or_insert((usize::MAX, usize::MIN));
            pair.0 = min(pair.0, index);
            pair.1 = max(pair.1, index);
        }


        let mut res = 0;
        let char_vec: Vec<char> = s.chars().collect();

        // For each character, find the count of unique character between positions calculated in last step.
        for (_, pair) in map {
            if pair.1 - pair.0 > 1 {
                let mut set = HashSet::new();

                for char in char_vec.iter().skip(pair.0 + 1).take(pair.1 - pair.0 - 1) {
                    if set.contains(char) {
                        continue;
                    }
                    set.insert(char);
                    res += 1;
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::count_palindromic_subsequence("aabca".to_string()), 3);
        assert_eq!(Solution::count_palindromic_subsequence("adc".to_string()), 0);
        assert_eq!(Solution::count_palindromic_subsequence("bbcbaba".to_string()), 4);
        assert_eq!(Solution::count_palindromic_subsequence("aaaaaaaaaaa".to_string()), 1);
    }
}