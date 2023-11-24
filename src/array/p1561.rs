//! 1561. Maximum Number of Coins You Can Get

use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    // Greedy
    pub fn max_coins(mut piles: Vec<i32>) -> i32 {
        piles.sort_unstable();
        let mut queue = VecDeque::from(piles);


        let mut res = 0;
        while !queue.is_empty() {
            queue.pop_back();
            res += queue.pop_back().unwrap();
            queue.pop_front();
        }
        res
    }

    fn no_queue(mut piles: Vec<i32>) -> i32 {
        piles.sort_unstable();

        let mut res = 0;
        let n = piles.len();
        for count in piles.into_iter().skip(n / 3).step_by(2).take(n / 3) {
            res += count;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::max_coins(vec![2, 4, 1, 2, 7, 8]), 9);
        assert_eq!(Solution::max_coins(vec![2, 4, 5]), 4);
        assert_eq!(Solution::max_coins(vec![9, 8, 7, 6, 5, 1, 2, 3, 4]), 18);
    }
}