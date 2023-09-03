///! 62. Unique Paths

struct Solution;

// The number of unique paths can be seen as the number of ways to choose m−1m-1m−1 downs and n−1n-1n−1 rights, regardless of the order.
// In combinatorial terms, this is equivalent to comb(m + n -2, m - 1) which is equal to comb(m + n -2, n - 1)
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut ans: i64 = 1;
        for i in 1..=m as i64 - 1 {
            ans = ans * (n as i64 - 1 + i) / i;
        }
        Self::combo(m + n - 2, m - 1)
    }

    fn combo(n: i32, m: i32) -> i32 {
        if n < m {
            panic!("n should be larger than m in combo calculating")
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
        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::unique_paths(1, 2), 1);
        assert_eq!(Solution::unique_paths(3, 2), 3);
        assert_eq!(Solution::unique_paths(4, 4), 20);
        assert_eq!(Solution::unique_paths(7, 3), 28);
    }
}