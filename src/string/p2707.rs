//! 2707. Extra Characters in a String

use std::cmp::min;
use std::collections::{HashMap, HashSet};

struct Solution;

struct DP {
    source: String,
    dictionary: HashSet<String>,
    // we need this cache because of starting search from the beginning of source string, aka, top down dynamic programming.
    // if we search from end to start, there is no duplicated calculating, and therefore no need for a cache
    cache: HashMap<usize /* index */, i32 /* result */>,
}

impl DP {
    fn dp(&mut self, start: usize, end: usize) -> i32 {
        if start == self.source.len() {
            return 0;
        }

        if let Some(count) = self.cache.get(&start) {
            return *count;
        }

        // result of next index add current character
        let mut count = self.dp(start + 1, end) + 1;
        // check each substring which start with index "start"
        for i in start + 1..=end {
            // check if range [start, i) is a valid word
            // TODO: use trie tree instead of substring
            if self.dictionary.contains(&self.source[start..i]) {
                // calculate result of next index of current word
                // And then take the smaller number as the final result
                count = min(count, self.dp(i, end));
            }
        }

        // cache result to prevent duplicated calculating
        self.cache.insert(start, count);
        count
    }
}

impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        // create DP struct
        let mut dp = DP {
            source: s.clone(),
            dictionary: HashSet::with_capacity(dictionary.len()),
            cache: HashMap::with_capacity(s.len()),
        };
        for str in dictionary {
            dp.dictionary.insert(str);
        }

        dp.dp(0, s.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::min_extra_char("ltscd".to_string(), vec!["lt".to_string(), "cd".to_string()]), 1);
        assert_eq!(Solution::min_extra_char("leetscode".to_string(), vec!["leet".to_string(), "code".to_string(), "leetcode".to_string()]), 1);
        assert_eq!(Solution::min_extra_char("sayhelloworld".to_string(), vec!["hello".to_string(), "world".to_string()]), 3);
    }
}