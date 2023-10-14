//! 2742. Painting the Walls

use std::cmp::min;
use std::collections::HashMap;

pub struct Solution;

struct DP {
    cost: Vec<i32>,
    time: Vec<i32>,
    cache: HashMap<(usize, i32), i32>,
}

impl DP {
    fn dp(&mut self, i: usize, remain: i32) -> i32 {
        // Similar to p746, return an illegal result if out of bound.
        if i >= self.cost.len() {
            return i32::MAX;
        }

        if let Some(cache) = self.cache.get(&(i, remain)) {
            return *cache;
        }

        // We have two choice: paint or not paint current wall by the paid painter.
        // Case 1: paint current wall by the paid painter. So that we get "1(by paid painter) + time(by free painter)" walls painted.
        let next_remain = remain - self.time[i] - 1;
        // We will directly return the cost of the paid painter if there are no remaining walls.
        let res_pick = if next_remain > 0 {
            let res = self.dp(i + 1, next_remain);
            // Avoid overflow.
            if res == i32::MAX {
                i32::MAX
            } else {
                self.cost[i] + res
            }
        } else {
            self.cost[i]
        };

        // Case 2: We not hire paid painter to paint current wall.
        let res_not_pick = self.dp(i + 1, remain);

        let res = min(res_pick, res_not_pick);
        self.cache.insert((i, remain), res);
        res
    }
}

impl Solution {
    pub fn paint_walls(cost: Vec<i32>, time: Vec<i32>) -> i32 {
        let mut dp = DP {
            cost,
            time,
            cache: HashMap::new(),
        };

        dp.dp(0, dp.cost.len() as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::paint_walls(vec![1], vec![1]), 1);
        assert_eq!(Solution::paint_walls(vec![1, 2, 3, 2], vec![1, 2, 3, 2]), 3);
        assert_eq!(Solution::paint_walls(vec![2, 3, 4, 2], vec![1, 1, 1, 1]), 4);
    }
}