//! 1425. Constrained Subsequence Sum

use std::cmp::max;
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn constrained_subset_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::with_capacity(nums.len());

        heap.push((nums[0], 0));
        let mut res = nums[0];


        // DP
        for (i, num) in nums.into_iter().enumerate().skip(1) {
            // Avoid skipping more than "k" numbers.
            // We cannot remove all of the elements k numbers before, which means we require O(n) space in the worst scenario.
            while i - heap.peek().unwrap().1 > k as usize {
                heap.pop();
            }

            // Add current number into last max value then push the result into heap.
            let current_value = max(0, heap.peek().unwrap().0) + num;
            heap.push((current_value, i));

            // In the previous section, we ensure that the current_value is the sum of a subarray
            // that does not skip more than k numbers.
            res = max(res, current_value);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // Notice: we can only choice [-1], [-1, -2] or [-2, -3], but not [-1, -3]
        assert_eq!(Solution::constrained_subset_sum(vec![-1, -2, -3], 1), -1);

        assert_eq!(Solution::constrained_subset_sum(vec![10, 2, -10, 5, 20], 2), 37);
        assert_eq!(Solution::constrained_subset_sum(vec![10, -2, -10, -5, 20], 2), 23);

        assert_eq!(Solution::constrained_subset_sum(vec![1, -2, -10, -5, 20], 2), 20);
        // Like case 1, we can choose [20].
        assert_eq!(Solution::constrained_subset_sum(vec![1, -2, -10, -5, 20], 4), 21);
    }
}