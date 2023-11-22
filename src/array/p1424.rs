//! 1424. Diagonal Traverse II

use std::collections::LinkedList;

pub struct Solution;

impl Solution {
    pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut index_queue = LinkedList::new();
        index_queue.push_back((0, 0));

        let mut res = vec![];

        // BFS
        while !index_queue.is_empty() {
            let (i, j) = index_queue.pop_front().unwrap();

            let num = &nums[i];
            res.push(num[j]);

            if j == 0 && i + 1 < nums.len() {
                index_queue.push_back((i + 1, j));
            }

            if j + 1 < num.len() {
                index_queue.push_back((i, j + 1));
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![vec![1], vec![2]];
        assert_eq!(Solution::find_diagonal_order(nums), vec![1, 2]);

        let nums = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(
            Solution::find_diagonal_order(nums),
            vec![1, 4, 2, 7, 5, 3, 8, 6, 9]
        );

        let nums = vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7],
            vec![8],
            vec![9, 10, 11],
            vec![12, 13, 14, 15, 16],
        ];
        assert_eq!(
            Solution::find_diagonal_order(nums),
            vec![1, 6, 2, 8, 7, 3, 9, 4, 12, 10, 5, 13, 11, 14, 15, 16]
        );

        let nums = vec![vec![1, 2, 3], vec![4], vec![5, 6, 7], vec![8], vec![9, 10, 11]];
        assert_eq!(
            Solution::find_diagonal_order(nums),
            vec![1, 4, 2, 5, 3, 8, 6, 9, 7, 10, 11]
        );

        let nums = vec![vec![1, 2, 3, 4, 5, 6]];
        assert_eq!(Solution::find_diagonal_order(nums), vec![1, 2, 3, 4, 5, 6]);
    }
}