//! 34. Find First and Last Position of Element in Sorted Array

use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    fn binary_search(nums: &Vec<i32>, target: i32) -> Option<i32> {
        if nums.is_empty() {
            return None;
        }

        let (mut left, mut right) = (0, nums.len());
        let mut mid = (left + right) / 2;
        while left < right {
            match nums[mid].cmp(&target) {
                Ordering::Less => left = mid + 1,
                Ordering::Equal => return Some(mid as i32),
                Ordering::Greater => right = mid,
            }
            mid = (left + right) / 2;
        }

        None
    }

    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        match Self::binary_search(&nums, target) {
            None => vec![-1, -1],
            Some(index) => {
                // search left
                let mut left = index as usize;
                while left > 0 && nums[left - 1] == target {
                    left -= 1;
                }
                // search right
                let mut right = index as usize;
                while right < nums.len() - 1 && nums[right + 1] == target {
                    right += 1;
                }
                vec![left as i32, right as i32]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8), vec![3, 4]);
        assert_eq!(Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6), vec![-1, -1]);
        assert_eq!(Solution::search_range(vec![], 0), vec![-1, -1]);
    }
}