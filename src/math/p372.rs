//! 372. Super Pow

pub struct Solution;

impl Solution {
    const MOD: u32 = 1337;

    // (x^n) mod m
    // See p50 for the "Binary Exponentiation" algorithm.
    // We can perform the modulo operation at each step without impacting the multiplication operation.
    fn bin_pow_mod(x: u32, n: u32, m: u32) -> u32 {
        if n == 0 {
            return 1;
        }

        let mut base = x % m;
        let mut exp = n;

        let mut res = 1;

        while exp > 0 {
            if exp & 1 == 1 {
                res = res * base % m;
            }

            base = base * base % m;
            exp >>= 1;
        }

        res
    }

    pub fn super_pow(a: i32, mut b: Vec<i32>) -> i32 {
        b.reverse();

        let mut res = 1;
        let mut base = a as u32 % Self::MOD;
        for exp in b {
            res = res * Self::bin_pow_mod(base, exp as u32, Self::MOD) % Self::MOD;
            base = Self::bin_pow_mod(base, 10, Self::MOD);
        }

        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::super_pow(2, vec![3]), 8);
        assert_eq!(Solution::super_pow(2, vec![1, 0]), 1024);
        assert_eq!(Solution::super_pow(1, vec![4, 3, 3, 8, 5, 2]), 1);
        assert_eq!(Solution::super_pow(2147483647, vec![2, 0, 0]), 1198);
        assert_eq!(Solution::super_pow(2147483647, vec![2, 0, 0, 0]), 407);
    }
}