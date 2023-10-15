//! 746. Min Cost Climbing Stairs

use std::cmp::min;

pub struct Solution;

struct DP {
    cost: Vec<i32>,
    cache: Vec<i32>,
}

impl DP {
    fn dp(&mut self, i: usize) -> i32 {
        // We use 1000 to exclude any out-of-bounds indexes, as the range for cost[i] is between 0 and 999.
        if i >= self.cost.len() {
            return 1000;
        }

        if i >= self.cost.len() - 2 {
            return self.cost[i];
        }

        if i > 1 && self.cache[i] != -1 {
            return self.cache[i];
        }

        let res = self.cost[i] + min(self.dp(i + 1), self.dp(i + 2));
        self.cache[i] = res;
        res
    }
}

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let len = cost.len();
        let mut dp = DP {
            cost,
            cache: vec![-1; len],
        };

        // Handle the case when the length of cost is equals to 2.
        dp.cache[0] = dp.cost[0];
        dp.cache[1] = dp.cost[1];


        dp.dp(0);

        min(dp.cache[0], dp.cache[1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::min_cost_climbing_stairs(vec![1, 2]), 1);
        assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20]), 15);
        assert_eq!(Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]), 6);
    }
}