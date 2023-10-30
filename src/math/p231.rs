//! 231. Power of Two

pub struct Solution;

impl Solution {
    // A number is a power of 2 if it has only one "1" in its binary representation,
    // and that "1" cannot be in the last bit.
    pub fn is_power_of_two(n: i32) -> bool {
        // See p191 for the explanation of the expression `n & (n - 1)`
        n > 0 && (n & (n - 1)) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::is_power_of_two(1), true);
        assert_eq!(Solution::is_power_of_two(16), true);
        assert_eq!(Solution::is_power_of_two(218), false);
    }
}