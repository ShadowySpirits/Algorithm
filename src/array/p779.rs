//! 779. K-th Symbol in Grammar

pub struct Solution;

impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        if n == 1 {
            return 0;
        }

        let mut deep = n - 1;
        let mut index = k - 1;
        let mut root = 0;

        while deep > 0 {
            let half_node_count = 2_i32.pow(deep as u32) / 2;

            // The root value is not changed if k is in the left subtree.
            if index >= half_node_count {
                // k is in the right subtree
                root = 1 - root;
                index -= half_node_count;
            }

            deep -= 1;
        }

        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::kth_grammar(1, 1), 0);
        assert_eq!(Solution::kth_grammar(2, 1), 0);
        assert_eq!(Solution::kth_grammar(2, 2), 1);
        assert_eq!(Solution::kth_grammar(4, 5), 1);

        assert_eq!(Solution::kth_grammar(6, 5), 1);
        assert_eq!(Solution::kth_grammar(6, 6), 0);
        assert_eq!(Solution::kth_grammar(6, 19), 0);
        assert_eq!(Solution::kth_grammar(6, 20), 1);
    }
}