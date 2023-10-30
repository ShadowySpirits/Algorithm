//! 191. Number of 1 Bits

pub struct Solution;

impl Solution {
    pub fn hamming_weight(n: u32) -> i32 {
        let mut n = n;
        let mut hamming_weight = 0;
        while n != 0 {
            // The expression `n & (n - 1)` flips the rightmost 1 bit to 0 bit.
            //
            // For example. 6(0110) & 5(0101) = 4(0100)
            // 0110
            // 0101
            // &---
            // 0100
            n = n & (n - 1);
            hamming_weight += 1;
        }
        hamming_weight
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::hamming_weight(0b00000000000000000000000000001011), 3);
        assert_eq!(Solution::hamming_weight(0b00000000000000000000000010000000), 1);
        assert_eq!(Solution::hamming_weight(0b11111111111111111111111111111101), 31);
    }
}