//! 377. Combination Sum IV

use std::cmp::Ordering;
use std::iter;

struct Solution;

struct DP {
    vec: Vec<i32>,
    res: i32,
}

impl DP {
    // WARNING: Time Limit Exceeded! Exhausting all combinations makes the time complexity too high
    // Order is matter, for example: (1, 1, 2) and (2, 1, 1) is the difference combination
    fn dp_permutation(&mut self, nums: &[i32], target: i32) {
        for i in 0..nums.len() {
            self.vec.push(nums[i]);
            match nums[i].cmp(&target) {
                Ordering::Less => self.dp_permutation(nums, target - nums[i]),
                Ordering::Equal => self.res += 1,
                _ => {}
            }
        }
    }

    // Order is not matter, for example: (1, 1, 2) and (2, 1, 1) is the same combination
    fn dp_combination(&mut self, nums: &[i32], target: i32) {
        for i in 0..nums.len() {
            match nums[i].cmp(&target) {
                Ordering::Less => self.dp_combination(&nums[i..], target - nums[i]),
                Ordering::Equal => self.res += 1,
                _ => {}
            }
        }
    }

    // The first two algorithms have an unfixed depth of recursion.
    // This method is built by the results of the recursive formula 0~target.
    fn dp(nums: Vec<i32>, target: i32) -> i32 {
        let mut res: Vec<i32> = iter::repeat(0).take(target as usize + 1).collect();
        for i in 1..=target {
            for num in nums.iter() {
                // Instead of listing all the possible combinations, we compute the outcome for each individual target.
                if *num < i && res[(i - num) as usize] > 0 {
                    res[i as usize] += res[(i - num) as usize];
                } else if *num == i {
                    res[i as usize] += 1;
                }
            }
        }
        res[target as usize]
    }

    pub fn dp_optimize(nums: Vec<i32>, target: i32) -> i32 {
        // Optimize the creation of the resource vector
        let mut dp = vec![0; (target + 1) as usize];
        // Set dp[0] to 1 so that we do not need to distinguish i == num or i < num
        dp[0] = 1;

        for i in 1..=target {
            for num in &nums {
                if *num <= i {
                    dp[i as usize] += dp[(i - num) as usize];
                }
            }
        }

        dp[target as usize]
    }
}

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        // let mut dp = DP {
        //     vec: vec![],
        //     res: 0,
        // };
        // dp.dp_permutation(&nums, target);
        // dp.res
        DP::dp(nums, target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::combination_sum4(vec![1, 2, 3], 4), 7);
        assert_eq!(Solution::combination_sum4(vec![2, 1, 3], 35), 1132436852);
    }
}