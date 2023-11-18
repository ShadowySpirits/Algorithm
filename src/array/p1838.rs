//! 1838. Frequency of the Most Frequent Element

pub struct Solution;

impl Solution {
    // Brute-force search with O(n^2) time complexity.
    pub fn max_frequency(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();

        let mut res = 0;
        for (index, target) in nums.iter().enumerate().rev() {
            let mut steak_count = 1;
            let mut credit = k;
            for num in nums.iter().rev().skip(nums.len() - index) {
                if target - num <= credit {
                    steak_count += 1;
                    credit -= target - num;
                } else {
                    break;
                }
            }

            res = res.max(steak_count);

            // Return early if the remaining elements cannot exceed the current result.
            if res > index {
                break;
            }
        }

        res as i32
    }

    // Using sliding window to reduce time complexity into O(nlog(n)).
    pub fn sliding_window(nums: Vec<i32>, k: i32) -> i32 {
        if nums.len() < 2 {
            return nums.len() as i32;
        }

        let mut nums = nums;
        // We sort the array to maintain an ascending order of the sliding window.
        // The time complexity of sort is O(nlog(n)).
        nums.sort();

        let mut res = 0;

        // We use two pointer left and right to represent the sliding window.
        let mut left = 0;
        let mut right = 1;

        let mut credit = k;
        let mut current_num = nums[0];

        // Because of left pointer can only move to the position of right pointer
        // and the max value of right pointer is nums.len().
        // So the time complexity of this loop is O(n).
        while right < nums.len() {
            let diff = nums[right] - current_num;

            // When the left pointer moves to the right, the credit will increase to k,
            // which is definitely greater than zero.
            while credit - diff * ((right - left) as i32) < 0 {
                credit += current_num - nums[left];
                left += 1;
            }

            credit -= diff * (right - left) as i32;
            res = res.max(right - left + 1);
            current_num = nums[right];
            right += 1;
        }

        res as i32
    }

    // Like line 27~29, if we found the max result, it is no need to reduce the window.
    // So that we can optimize the inner loop (line 63~66) into a simple "if" statement.
    pub fn advance_sliding_window(nums: Vec<i32>, k: i32) -> i32 {
        if nums.len() < 2 {
            return nums.len() as i32;
        }

        let mut nums = nums;
        nums.sort();

        let mut left = 0;
        let mut sum = 0;

        for right in 0..nums.len() {
            sum += nums[right];

            // Move both left and right pointer into the next position to keep the window size.
            if ((right - left + 1) as i32) * nums[right] - sum > k {
                sum -= nums[left];
                left += 1;
            }
        }

        (nums.len() - left) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::max_frequency(vec![], 5), 0);
        assert_eq!(Solution::max_frequency(vec![1], 5), 1);

        assert_eq!(Solution::max_frequency(vec![1, 2], 5), 2);
        assert_eq!(Solution::max_frequency(vec![1, 1000], 5), 1);


        assert_eq!(Solution::max_frequency(vec![1, 2, 4], 5), 3);
        assert_eq!(Solution::max_frequency(vec![1, 4, 8, 13], 5), 2);
        assert_eq!(Solution::max_frequency(vec![3, 9, 6], 2), 1);

        assert_eq!(Solution::max_frequency(vec![1, 2, 3, 4], 6), 4);
        assert_eq!(Solution::max_frequency(vec![0, 1, 2, 3, 4, 1000, 1000], 6), 4);
    }
}