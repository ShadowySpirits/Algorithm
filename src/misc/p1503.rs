//! 1503. Last Moment Before All Ants Fall Out of a Plank

use std::cmp::max;

pub struct Solution;

impl Solution {
    pub fn get_last_moment(n: i32, left: Vec<i32>, right: Vec<i32>) -> i32 {
        if !left.is_empty() && !right.is_empty() {
            let left = left.into_iter().max().unwrap();
            let right = right.into_iter().min().unwrap();

            return max(left, n - right);
        }

        if left.is_empty() {
            return n - right.into_iter().min().unwrap();
        }

        left.into_iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::get_last_moment(4, vec![4, 3], vec![0, 1]), 4);
        assert_eq!(Solution::get_last_moment(9, vec![5], vec![4]), 5);
        assert_eq!(Solution::get_last_moment(6, vec![6], vec![0]), 6);
        assert_eq!(Solution::get_last_moment(4, vec![0, 1], vec![3, 4]), 1);
        assert_eq!(Solution::get_last_moment(7, vec![1, 6], vec![0, 2, 3, 4, 5, 7]), 7);


        assert_eq!(Solution::get_last_moment(7, vec![], vec![0, 1, 2, 3, 4, 5, 6, 7]), 7);
        assert_eq!(Solution::get_last_moment(7, vec![0, 1, 2, 3, 4, 5, 6, 7], vec![]), 7);
        assert_eq!(Solution::get_last_moment(1000, vec![], vec![1000]), 0);
        assert_eq!(Solution::get_last_moment(1000, vec![1000], vec![]), 1000);
    }
}