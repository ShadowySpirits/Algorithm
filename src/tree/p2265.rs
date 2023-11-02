//! 2265. Count Nodes Equal to Average of Subtree

use std::cell::RefCell;
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

struct DFS {
    res: i32,
}

impl DFS {
    fn postorder_traverse(&mut self, root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        return if let Some(root) = root {
            let root = root.borrow();

            let (left_sum, left_count) = self.postorder_traverse(&root.left);
            let (right_sum, right_count) = self.postorder_traverse(&root.right);

            let sum = root.val + left_sum + right_sum;
            let count = 1 + left_count + right_count;

            if sum / count == root.val {
                self.res += 1;
            }

            (sum, count)
        } else {
            (0, 0)
        };
    }
}

impl Solution {
    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut dfs = DFS {
            res: 0,
        };
        dfs.postorder_traverse(&root);
        dfs.res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let root = TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
        };
        assert_eq!(Solution::average_of_subtree(Some(Rc::new(RefCell::new(root)))), 3);

        let root = TreeNode {
            val: 8,
            left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        };
        assert_eq!(Solution::average_of_subtree(Some(Rc::new(RefCell::new(root)))), 2);
    }
}