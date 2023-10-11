//! 2009. Minimum Number of Operations to Make Array Continuous

use std::cmp::{min, Ordering};
use std::collections::BTreeSet;

pub struct Solution;

impl Solution {
    /// Returns the index of a number that is less than or equal to a given upper bound.
    /// Returns None if the upper bound is less than all numbers in nums.
    pub fn binary_search(nums: &Vec<i32>, upper_bound: i32) -> Option<usize> {
        let (mut left, mut right) = (0, nums.len());
        let mut mid = (left + right) / 2;

        while left < right {
            match nums[mid].cmp(&upper_bound) {
                Ordering::Less => left = mid + 1,
                Ordering::Equal => return Some(mid),
                Ordering::Greater => right = mid,
            }
            mid = (left + right) / 2;
        }

        if mid == 0 {
            None
        } else {
            Some(mid - 1)
        }
    }

    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let len = nums.len() as i32;

        // Filter out duplicated number and sort the map
        let mut set = BTreeSet::new();
        for num in nums {
            set.insert(num);
        }

        let vec: Vec<i32> = set.into_iter().collect();

        // Iterate all numbers to find out min operation count.
        // Treat each number as the lower bound, and find how many numbers in range [num, num + len - 1]
        let mut res = i32::MAX;
        for (left, num) in vec.iter().enumerate() {
            // Use binary search to find the index of the number less than or equal to upper bound.
            let upper_bound = num + len - 1;
            match Self::binary_search(&vec, upper_bound) {
                // If the index is exist, treat it as the right bound.
                // So that the number in range [left, right] do not need
                Some(right) => res = min(res, len - (right - left + 1) as i32),
                // If the right bound is not found, it means all other numbers need to be changed.
                None => res = min(res, len - 1)
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
        assert_eq!(Solution::min_operations(vec![4, 2, 5, 3]), 0);
        assert_eq!(Solution::min_operations(vec![1, 2, 3, 5, 6]), 1);
        assert_eq!(Solution::min_operations(vec![1, 10, 100, 1000]), 3);
    }

    #[test]
    fn test_binary_search() {
        assert_eq!(Solution::binary_search(&vec![1], 0), None);
        assert_eq!(Solution::binary_search(&vec![1, 2], 0), None);

        assert_eq!(Solution::binary_search(&vec![1, 2, 4], 3), Some(1));
        assert_eq!(Solution::binary_search(&vec![0, 1, 2, 4], 3), Some(2));
        assert_eq!(Solution::binary_search(&vec![1, 2, 3, 4], 3), Some(2));

        assert_eq!(Solution::binary_search(&vec![1, 2, 3, 4], 5), Some(3));
        assert_eq!(Solution::binary_search(&vec![1, 2, 3, 4, 5], 10), Some(4));
    }
}