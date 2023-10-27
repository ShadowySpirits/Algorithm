//! 844. Backspace String Compare

pub struct Solution;

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let mut s_stack = vec![];
        for char in s.chars() {
            if char == '#' {
                s_stack.pop();
                continue;
            }
            s_stack.push(char);
        }

        let mut t_stack = vec![];
        for char in t.chars() {
            if char == '#' {
                t_stack.pop();
                continue;
            }
            t_stack.push(char);
        }

        s_stack == t_stack
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::backspace_compare("#".to_string(), "".to_string()), true);
        assert_eq!(Solution::backspace_compare("ab#c".to_string(), "ad#c".to_string()), true);
        assert_eq!(Solution::backspace_compare("ab##".to_string(), "c#d#".to_string()), true);
        assert_eq!(Solution::backspace_compare("a##c".to_string(), "#a#c".to_string()), true);
        assert_eq!(Solution::backspace_compare("a#c".to_string(), "b".to_string()), false);
    }
}