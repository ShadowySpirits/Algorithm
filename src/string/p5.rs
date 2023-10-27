//! 5. Longest Palindromic Substring

pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut max_len = 1;
        let mut left = 0;
        let mut right = 0;

        let chars: Vec<char> = s.chars().collect();

        // Find all palindrome substring with odd length.
        for (i, _char) in chars.iter().enumerate().skip(1) {
            let mut cur_left = i;
            let mut cur_right = i;

            while cur_left > 0 && cur_right < chars.len() - 1 && chars[cur_left - 1] == chars[cur_right + 1] {
                cur_left -= 1;
                cur_right += 1;
            }

            if cur_right - cur_left + 1 > max_len {
                left = cur_left;
                right = cur_right;
                max_len = right - left + 1;
            }
        }

        // Find all palindrome substring with even length.
        for (i, _char) in chars.iter().enumerate().skip(1) {
            let mut cur_left = i - 1;
            let mut cur_right = i;

            if chars[cur_left] != chars[cur_right] {
                continue;
            }

            while cur_left > 0 && cur_right < chars.len() - 1 && chars[cur_left - 1] == chars[cur_right + 1] {
                cur_left -= 1;
                cur_right += 1;
            }

            if cur_right - cur_left + 1 > max_len {
                left = cur_left;
                right = cur_right;
                max_len = right - left + 1;
            }
        }

        s[left..=right].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::longest_palindrome("babad".to_string()), "bab".to_string());
        assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb".to_string());
        assert_eq!(Solution::longest_palindrome("a".to_string()), "a".to_string());
        assert_eq!(Solution::longest_palindrome("ac".to_string()), "a".to_string());

        assert_eq!(Solution::longest_palindrome("aaa".to_string()), "aaa".to_string());
        assert_eq!(Solution::longest_palindrome("aaaa".to_string()), "aaaa".to_string());
        assert_eq!(Solution::longest_palindrome("aaaaa".to_string()), "aaaaa".to_string());
        assert_eq!(Solution::longest_palindrome("abbaa".to_string()), "abba".to_string());
        assert_eq!(Solution::longest_palindrome("aabbaaa".to_string()), "aabbaa".to_string());

        assert_eq!(Solution::longest_palindrome("cbabc".to_string()), "cbabc".to_string());
        assert_eq!(Solution::longest_palindrome("cbaabc".to_string()), "cbaabc".to_string());
        assert_eq!(Solution::longest_palindrome("dcbaabce".to_string()), "cbaabc".to_string());

        assert_eq!(Solution::longest_palindrome("abbcccbbbcaaccbababcbcabca".to_string()), "bbcccbb".to_string());

        assert_eq!(Solution::longest_palindrome("bananas".to_string()), "anana".to_string());
        assert_eq!(Solution::longest_palindrome("ababababababa".to_string()), "ababababababa".to_string());
    }
}