//! 458. Poor Pigs

pub struct Solution;

impl Solution {
    // See https://zhuanlan.zhihu.com/p/74663700
    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        let base = (minutes_to_test / minutes_to_die + 1) as f32;
        ((buckets as f32).log2() / base.log2()).ceil() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::poor_pigs(1000, 15, 60), 5);
        assert_eq!(Solution::poor_pigs(4, 15, 15), 2);
        assert_eq!(Solution::poor_pigs(4, 15, 30), 2);
    }
}