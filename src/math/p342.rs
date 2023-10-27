//! 342. Power of Four

pub struct Solution;

impl Solution {
    // A number is a power of 4 if:
    // 1. it is power of 2, see p231
    // 2. the "1" bit is in an odd position
    pub fn is_power_of_four(n: i32) -> bool {
        // The number 0x55555555 has "1" in all odd positions and "0" in all even positions.
        let mask = 0x55555555;
        n > 0 && (n & (n - 1)) == 0 && (n & mask) == n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::is_power_of_four(1), true);
        assert_eq!(Solution::is_power_of_four(16), true);
        assert_eq!(Solution::is_power_of_four(5), false);
    }
}