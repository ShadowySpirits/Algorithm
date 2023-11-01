//! 501. Find Mode in Binary Search Tree

use std::cell::RefCell;
use std::cmp::Ordering;
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
    res: Vec<i32>,
    last_value: i32,
    last_count: i32,
    max_count: i32,
}

impl DFS {
    fn handle_last_value(&mut self) {
        match self.last_count.cmp(&self.max_count) {
            Ordering::Less => {}
            Ordering::Equal => self.res.push(self.last_value),
            Ordering::Greater => {
                self.max_count = self.last_count;
                self.res.clear();
                self.res.push(self.last_value);
            }
        }
    }

    fn inorder_traverse(&mut self, root: Option<Rc<RefCell<TreeNode>>>) {
        if let Some(root) = root {
            let mut root = root.borrow_mut();

            self.inorder_traverse(root.left.take());

            // Perform an inorder DFS traversal.
            // If this code before line 52, it is a preorder traversal.
            // If this code after line 65, it is a postorder traversal.
            if root.val == self.last_value {
                self.last_count += 1;
            } else {
                self.handle_last_value();
                self.last_value = root.val;
                self.last_count = 1;
            }

            self.inorder_traverse(root.right.take());
        }
    }
}

impl Solution {
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut dfs = DFS {
            res: vec![],
            last_value: -1,
            last_count: -1,
            max_count: 0,
        };

        dfs.inorder_traverse(root);
        dfs.handle_last_value();

        dfs.res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let root = TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        };
        let expected = vec![1, 2];
        assert_eq!(Solution::find_mode(Some(Rc::new(RefCell::new(root)))), expected);

        let right = TreeNode {
            val: 2,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        };
        let root = TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(right))),
        };
        let expected = vec![2];
        assert_eq!(Solution::find_mode(Some(Rc::new(RefCell::new(root)))), expected);
    }
}