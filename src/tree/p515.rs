//! 515. Find Largest Value in Each Tree Row

use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution;

impl Solution {
    fn dfs(node: Rc<RefCell<TreeNode>>, level: usize, res: &mut Vec<i32>) {
        let node = node.borrow();
        if res.len() == level {
            res.push(node.val);
        } else {
            res[level] = max(res[level], node.val);
        }

        if let Some(left) = node.left.clone() {
            Self::dfs(left, level + 1, res);
        }

        if let Some(right) = node.right.clone() {
            Self::dfs(right, level + 1, res);
        }
    }

    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        if let Some(root) = root {
            Self::dfs(root, 0, &mut res);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let result = Solution::largest_values(root);
        assert_eq!(result, vec![1]);

        let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        root.as_ref().unwrap().borrow_mut().left = left;
        root.as_ref().unwrap().borrow_mut().right = right;
        let result = Solution::largest_values(root);
        assert_eq!(result, vec![1, 3]);

        let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let left_left = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        let left_right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let right_right = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        left.as_ref().unwrap().borrow_mut().left = left_left;
        left.as_ref().unwrap().borrow_mut().right = left_right;
        right.as_ref().unwrap().borrow_mut().right = right_right;
        root.as_ref().unwrap().borrow_mut().left = left;
        root.as_ref().unwrap().borrow_mut().right = right;
        let result = Solution::largest_values(root);
        assert_eq!(result, vec![1, 3, 9]);
    }
}