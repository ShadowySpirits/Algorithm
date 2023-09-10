//! 1359. Count All Valid Pickup and Delivery Options

struct Solution;

impl Solution {
    const MOD: u128 = 1_000_000_007;

    // Obviously, we need to solve this problem via dynamic programming.
    //
    // Treat "Pickup" and "Delivery" as an ordered sequence, for example, (P1, D1) and (P2, D2).
    // The result of count_orders(2) is equivalent to inserting (P2, D2) into the sequence of count_orders(2).
    // Therefore, we need to calculate the total count of all combinations, instead of permutations, of (P2, D2) inserted in the previous sequence.
    //
    // Hence, the general formula is count_orders(n) = count_orders(n - 1) * (combo((n - 1) * 2 + 1, 1) + combo((n - 1) * 2 + 1, 2))
    pub fn count_orders(n: i32) -> i32 {
        let mut dp = vec![0u128; (n + 1) as usize];
        dp[0] = 1;

        for i in 1..=n as usize {
            // Total possible inserted position of last result.
            let n = (i - 1) * 2 + 1;
            // Calculate combo(n, 1) plus combo(n, 2).
            let cn1_plus_cn2 = Self::combo(n, 1) + Self::combo(n, 2);
            dp[i] = dp[i - 1] * cn1_plus_cn2 % Self::MOD
        }

        dp[n as usize] as i32
    }

    // modified according to crate::math::p62::Solution::combo()
    pub fn combo(n: usize, m: usize) -> u128 {
        if n < m {
            return 0
        }

        let n = n as u128;
        let mut m = m as u128;

        // combo(n, m) is equal to combo(n, n - m)
        if 2 * m > n {
            m = n - m
        };

        let mut res = 1u128;
        for i in 1..=m {
            res = res * (n - m + i) / i;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::count_orders(1), 1);
        assert_eq!(Solution::count_orders(2), 6);
        assert_eq!(Solution::count_orders(3), 90);
        assert_eq!(Solution::count_orders(4), 2520);
    }
}