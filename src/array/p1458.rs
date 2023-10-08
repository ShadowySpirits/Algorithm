//! 1458. Max Dot Product of Two Subsequences

use std::cmp::max;

pub struct Solution;

struct DP {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
    cache: Vec<Vec<i32>>,
}

impl DP {
    fn dp(&mut self, i: usize, j: usize) -> i32 {
        if i >= self.nums1.len() || j >= self.nums2.len() {
            return i32::MIN;
        }

        if self.cache[i][j] != i32::MIN {
            return self.cache[i][j];
        }

        // Case1: Calculate the dot product of current numbers and all the rest numbers.
        // Case2: Calculate the dot product of current number of array "nums1" and all the rest numbers.
        // Case3: Calculate the dot product of current number of array "nums2" and all the rest numbers.
        self.cache[i][j] = max(
            // Notice: if the result of rest numbers is negative, we should ignore it to prevent overflow during addition.
            // Which means we should only calculate dot product of current numbers.
            self.nums1[i] * self.nums2[j] + max(self.dp(i + 1, j + 1), 0),
            // Call dp function recursively to calculate case 2 and case 3.
            max(self.dp(i + 1, j), self.dp(i, j + 1)),
        );

        // Save calculation results to cache array.
        self.cache[i][j]
    }
}

impl Solution {
    pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let num1_len = nums1.len();
        let num2_len = nums2.len();
        let mut dp = DP {
            nums1,
            nums2,
            cache: vec![vec![i32::MIN; num2_len]; num1_len],
        };


        dp.dp(0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::max_dot_product(vec![2, 1, -2, 5], vec![3, 0, -6]), 18);
        assert_eq!(Solution::max_dot_product(vec![3, -2], vec![2, -6, 7]), 21);
        assert_eq!(Solution::max_dot_product(vec![-1, -1], vec![1, 1]), -1);

        // If do not use memory array to cache result, it will cause TLE.
        assert_eq!(Solution::max_dot_product(vec![13,-7,12,-15,-7,8,3,-7,-5,13,-15,-8,5,7,-1,3,-11,-12,2,-12], vec![-1,13,-4,-2,-13,2,-4,6,-9,13,-8,-3,-9]), 972);
    }
}