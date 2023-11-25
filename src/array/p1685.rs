//! 1685. Sum of Absolute Differences in a Sorted Array

pub struct Solution;

impl Solution {
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len() as i32;

        let mut left_sum = 0;
        let mut right_sum: i32 = nums.iter().sum();

        let mut res = vec![];
        for (index, num) in nums.iter().enumerate() {
            let index = index as i32;
            res.push(num * index - left_sum + right_sum - num * (n - index));

            left_sum += num;
            right_sum -= num;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::get_sum_absolute_differences(vec![2, 3, 5]), vec![4, 3, 5]);
        assert_eq!(Solution::get_sum_absolute_differences(vec![1, 4, 6, 8, 10]), vec![24, 15, 13, 15, 21]);
    }
}