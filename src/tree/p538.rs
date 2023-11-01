//! 538. Convert BST to Greater Tree

use std::rc::Rc;
use std::cell::RefCell;

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
    sum: i32,
}

impl DFS {

    // Similar with p505, but traversing in descending order.
    // The problem is reassign each node to "sum of all keys greater than the original key".
    // Thus, we traverse all key in descending order, calculate the summary, and assign it to each node.
    fn inorder_traverse(&mut self, root: &Option<Rc<RefCell<TreeNode>>>) {
        if let Some(root) = root {
            let mut root = root.borrow_mut();

            // To traverse in descending order, start by accessing the right child.
            self.inorder_traverse(&root.right);

            self.sum += root.val;
            root.val = self.sum;

            self.inorder_traverse(&root.left);
        }
    }
}
impl Solution {
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut dfs = DFS {
            sum: 0
        };

        dfs.inorder_traverse(&root);

        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        let left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let right = Some(Rc::new(RefCell::new(TreeNode::new(13))));
        root.as_ref().unwrap().borrow_mut().left = left;
        root.as_ref().unwrap().borrow_mut().right = right;
        let root = Solution::convert_bst(root);
        let root = root.as_ref().unwrap().borrow();
        assert_eq!(root.val, 18);
        assert_eq!(root.left.as_ref().unwrap().borrow().val, 20);
        assert_eq!(root.right.as_ref().unwrap().borrow().val, 13);
    }
}