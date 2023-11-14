//! 1759. Count Number of Homogenous Substrings

pub struct Solution;

impl Solution {
    // A n length string has `(n + 1) * n / 2` non-empty substring and `1` empty substring.
    fn count_substring(n: i32) -> i32 {
        let n = n as u128;
        ((n + 1) * n / 2 % 1_000_000_007) as i32
    }

    // For each character sequence, we count the number of substrings.
    pub fn count_homogenous(s: String) -> i32 {
        let mut res = 0;

        let mut count = 0;
        let mut prev = ' ';
        for char in s.chars() {
            if char != prev {
                res = (res + Self::count_substring(count)) % 1_000_000_007;
                count = 1;
                prev = char;
            } else {
                count += 1;
            }
        }

        (res + Self::count_substring(count)) % 1_000_000_007
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::count_homogenous("abbcccaa".to_string()), 13);
        assert_eq!(Solution::count_homogenous("xy".to_string()), 2);
        assert_eq!(Solution::count_homogenous("zzzzz".to_string()), 15);
        assert_eq!(Solution::count_homogenous("zzzzzzz".to_string()), 28);
    }
}