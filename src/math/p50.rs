//! 50. Pow(x, n)

pub struct Solution;

impl Solution {
    // Binary Exponentiation, see https://oi-wiki.org/math/binary-exponentiation.
    // We can represent n as binary, for example, x is 5 and n is 13:
    // x^13 = x^(1101) = x^8 * x^4 * x^1
    // Notice: we skip the x^2.
    // So that we can do this task in O(log(n)) time complexity.
    pub fn bin_pow(x: f64, n: u32) -> f64 {
        if n == 0 {
            return 1.0;
        }

        // Initialize the base as x^1.
        let mut base = x;
        let mut exp = n;

        let mut res = 1.0;

        while exp > 0 {
            // If the current bit is 1, we can multiply base to the result.
            if exp & 1 == 1 {
                res *= base;
            }

            // base = base^2
            base *= base;
            // Check higher bit in next loop.
            exp >>= 1;
        }

        res
    }

    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n > 0 {
            Self::bin_pow(x, n as u32)
        } else {
            1.0 / Self::bin_pow(x, n.unsigned_abs())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::my_pow(2.0, 10), 1024.0);
        assert_eq!(Solution::my_pow(2.1, 3), 9.261000000000001);
        assert_eq!(Solution::my_pow(2.0, -2), 0.25);
        assert_eq!(Solution::my_pow(0.00001, 2147483647), 0.0);
    }
}