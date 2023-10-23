//! 1793. Maximum Score of a Good Subarray

use std::cmp::{max, min};

pub struct Solution;

impl Solution {
    fn dfs_with_greedy(nums: &Vec<i32>, i: i32, j: i32, min_value: i32) -> i32 {
        if i < 0 || j >= nums.len() as i32 {
            return -1;
        }

        let res = min_value * (j - i + 1);

        let left_min = if i > 0 {
            min(min_value, nums[i as usize - 1])
        } else { -1 };

        let right_min = if j + 1 < nums.len() as i32 {
            min(min_value, nums[j as usize + 1])
        } else { -1 };

        // Implement greedy pruning in the depth-first search (DFS) algorithm
        // by always choosing the larger number because of the equals weight of two path.
        let next_res = if left_min > right_min {
            Self::dfs_with_greedy(nums, i - 1, j, left_min)
        } else {
            Self::dfs_with_greedy(nums, i, j + 1, right_min)
        };

        max(res, next_res)
    }

    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        Self::dfs_with_greedy(&nums, k, k, nums[k as usize])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::maximum_score(vec![10], 0), 10);
        assert_eq!(Solution::maximum_score(vec![1, 4, 3, 7, 4, 5], 3), 15);
        assert_eq!(Solution::maximum_score(vec![5, 5, 4, 5, 4, 1, 1, 1], 0), 20);
    }
}