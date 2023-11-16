//! 1980. Find Unique Binary String

use std::collections::HashSet;

pub struct Solution;

impl Solution {
    // See https://leetcode.com/problems/find-unique-binary-string/editorial for more solutions.
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let n = nums.len();
        let mut set = HashSet::new();

        for num in nums {
            let num_radix_10 = usize::from_str_radix(&num, 2).unwrap();
            set.insert(num_radix_10);
        }

        let mut res = "".to_string();
        for i in 0..=n {
            if !set.contains(&i) {
                res = format!("{:b}", i)
            }
        }

        if res.len() < n {
            res = "0".repeat(n - res.len()) + &res;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec!["01".to_string(), "10".to_string()];
        assert_eq!(Solution::find_different_binary_string(nums), "00");

        let nums = vec!["00".to_string(), "01".to_string()];
        assert_eq!(Solution::find_different_binary_string(nums), "10");

        let nums = vec!["00".to_string(), "10".to_string()];
        assert_eq!(Solution::find_different_binary_string(nums), "01");

        let nums = vec!["111".to_string(), "011".to_string(), "001".to_string()];
        assert_eq!(Solution::find_different_binary_string(nums), "010");

        let nums = vec!["000".to_string(), "011".to_string(), "110".to_string()];
        assert_eq!(Solution::find_different_binary_string(nums), "010");
    }
}