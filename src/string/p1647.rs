//! 1647. Minimum Deletions to Make Character Frequencies Unique

use std::collections::{BTreeMap, HashMap};

pub struct Solution;

impl Solution {
    pub fn min_deletions(s: String) -> i32 {
        // Map<char, char_repeat_count>
        let mut char_map = HashMap::new();
        for char in s.chars() {
            let repeat_count = char_map.entry(char).or_insert(0);
            *repeat_count += 1;
        }

        // Map<repeat_count, letter_count>
        // We need a heap to get how many letter has the largest repeat count.
        // BinaryHeap is already okay, but I use the BTreeMap to simplify the code for retrieving values.
        let mut count_map = BTreeMap::new();
        for (_, repeat_count) in char_map {
            let letter_count = count_map.entry(repeat_count).or_insert(0);
            *letter_count += 1;
        }

        let mut delete_count = 0;

        // Iterating repeat count from max to 0 to calculate the delete count.
        //
        // Rust runtime version in leetcode is 1.58.2, which does not support revive first or last in the tree map.
        // We can use "pop_last" instead of "next_back" and "remove" in higher MSRV.
        while let Some((repeat_count, letter_count)) = count_map.iter().next_back() {
            let repeat_count = *repeat_count;
            let letter_count = *letter_count;

            if repeat_count == 0 {
                break;
            }

            if letter_count > 1 {
                // If the letter count exceeds 1, we need to keep only one letter, and delete the rest.
                // So, in this loop, the delete count is  letter_count - 1.
                *count_map.entry(repeat_count - 1).or_insert(0) += letter_count - 1;
                delete_count += letter_count - 1;
            }
            count_map.remove(&repeat_count);
        }

        delete_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::min_deletions("aab".to_string()), 0);
        assert_eq!(Solution::min_deletions("aaabbbcc".to_string()), 2);
        assert_eq!(Solution::min_deletions("ceabaacb".to_string()), 2);
        assert_eq!(Solution::min_deletions("bbcebab".to_string()), 2);
        assert_eq!(Solution::min_deletions("abcabc".to_string()), 3);
        assert_eq!(Solution::min_deletions("accdcdadddbaadbc".to_string()), 1);
    }
}