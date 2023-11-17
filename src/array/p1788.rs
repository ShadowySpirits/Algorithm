//! 1877. Minimize Maximum Pair Sum in Array

pub struct Solution;

impl Solution {
    pub fn min_pair_sum(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut nums = nums;
        nums.sort();

        let mut res = 0;
        for i in 0..n / 2 {
            res = res.max(nums[i] + nums[n - i - 1])
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::min_pair_sum(vec![3, 5, 4, 2, 4, 6]), 8);
        assert_eq!(Solution::min_pair_sum(vec![3, 5, 2, 3]), 7);
    }
}