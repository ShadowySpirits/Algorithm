//! 135. Candy

pub struct Solution;

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut candy_dist = vec![1; ratings.len()];

        for i in 1..ratings.len() {
            if ratings[i] > ratings[i - 1] {
                candy_dist[i] = candy_dist[i - 1] + 1
            }
        }

        for i in (0..(ratings.len() - 1)).rev() {
            if ratings[i] > ratings[i + 1] {
                candy_dist[i] = std::cmp::max(candy_dist[i], candy_dist[i + 1] + 1)
            }
        }

        candy_dist.iter().sum()
    }

    // TODO
    fn one_pass_greedy() {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::candy(vec![1, 0, 2]), 5);
        assert_eq!(Solution::candy(vec![1, 2, 2]), 4);
        assert_eq!(Solution::candy(vec![4, 3, 2, 1]), 10);
        assert_eq!(Solution::candy(vec![1, 2, 3, 4]), 10);
        assert_eq!(Solution::candy(vec![1, 2, 3, 0, 4]), 9);

        // [1, 2, 1, 2, 1]
        assert_eq!(Solution::candy(vec![1, 3, 2, 2, 1]), 7);
        // [1, 2, 3, 1, 3, 2, 1]
        assert_eq!(Solution::candy(vec![1, 2, 87, 87, 87, 2, 1]), 13);
    }
}