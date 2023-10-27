//! 507. Perfect Number

pub struct Solution;

impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        if num <= 1 {
            return false;
        }

        let mut vec = vec![];
        let sqrt = (num as f32).sqrt() as i32;
        for i in 2..=sqrt {
            if num % i == 0 {
                vec.push(i);
                vec.push(num / i);
            }
        }
        let sum: i32 = vec.iter().sum();
        sum + 1 == num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::check_perfect_number(28), true);
        assert_eq!(Solution::check_perfect_number(6), true);
        assert_eq!(Solution::check_perfect_number(496), true);
        assert_eq!(Solution::check_perfect_number(8128), true);
        assert_eq!(Solution::check_perfect_number(2), false);
        assert_eq!(Solution::check_perfect_number(1), false);
    }
}