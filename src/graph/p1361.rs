//! 1361. Validate Binary Tree Nodes

use std::collections::HashSet;

pub struct Solution;

impl Solution {
    fn find_root(n: i32, left_child: &Vec<i32>, right_child: &Vec<i32>) -> Option<i32> {
        let mut set = HashSet::with_capacity(n as usize);
        for node in vec![left_child, right_child].into_iter().flatten() {
            set.insert(*node);
        }

        (0..n).find(|&i| !set.contains(&i))
    }

    fn dfs(node: i32, left_child: &Vec<i32>, right_child: &Vec<i32>, visit: &mut HashSet<i32>) -> bool {
        if visit.contains(&node) {
            return false;
        }
        visit.insert(node);

        let left = left_child[node as usize];
        let right = right_child[node as usize];

        let left_result = if left != -1 {
            Self::dfs(left, left_child, right_child, visit)
        } else { true };

        let right_result = if right != -1 {
            Self::dfs(right, left_child, right_child, visit)
        } else { true };

        left_result && right_result
    }

    pub fn validate_binary_tree_nodes(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
        let root = Solution::find_root(n, &left_child, &right_child);
        if root.is_none() {
            return false;
        }
        let root = root.unwrap();

        let mut visit = HashSet::with_capacity(n as usize);

        let dfs_result = Solution::dfs(root, &left_child, &right_child, &mut visit);
        dfs_result && visit.len() == n as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::bool_assert_comparison)]
    fn test() {
        assert_eq!(Solution::validate_binary_tree_nodes(4, vec![1, -1, 3, -1], vec![2, -1, -1, -1]), true);
        assert_eq!(Solution::validate_binary_tree_nodes(4, vec![1, -1, 3, -1], vec![2, 3, -1, -1]), false);
        assert_eq!(Solution::validate_binary_tree_nodes(2, vec![1, 0], vec![-1, -1]), false);
        assert_eq!(Solution::validate_binary_tree_nodes(6, vec![1, -1, -1, 4, -1, -1], vec![2, -1, -1, 5, -1, -1]), false);
        assert_eq!(Solution::validate_binary_tree_nodes(5, vec![4, -1, 3, -1, -1], vec![1, -1, -1, 4, -1]), false);
    }
}