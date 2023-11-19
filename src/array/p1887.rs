//! 1887. Reduction Operations to Make the Array Elements Equal

pub struct Solution;

impl Solution {
    pub fn reduction_operations(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        let mut res = 0;

        let mut last_num = -1;
        let mut num_count = -1;
        let mut steak_count = 0;

        for num in nums {
            if num != last_num {
                res += num_count * steak_count;
                num_count += 1;
                last_num = num;
                steak_count = 1;
            } else {
                steak_count += 1;
            }
        }
        res += num_count * steak_count;

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![5, 1, 3];
        let ans = 3;
        assert_eq!(Solution::reduction_operations(nums), ans);

        let nums = vec![1, 1, 1];
        let ans = 0;
        assert_eq!(Solution::reduction_operations(nums), ans);

        let nums = vec![1, 1, 2, 2, 3];
        let ans = 4;
        assert_eq!(Solution::reduction_operations(nums), ans);
    }
}