//! 1269. Number of Ways to Stay in the Same Place After Some Steps

use std::collections::HashMap;

pub struct Solution;

struct DP {
    steps: i32,
    arr_len: i32,
    cache: HashMap<(i32, i32), u128>,
}

impl DP {
    const MOD: u128 = 1_000_000_007;

    fn dp(&mut self, step: i32, index: i32) -> u128 {
        if index < 0 || index >= self.arr_len {
            return 0;
        }

        if step > self.steps {
            return if index == 0 { 1 } else { 0 };
        }

        if let Some(cache) = self.cache.get(&(step, index)) {
            return *cache;
        }


        let left = self.dp(step + 1, index - 1);
        let stay = self.dp(step + 1, index);
        let right = self.dp(step + 1, index + 1);

        let res = (left + stay + right) % Self::MOD;
        self.cache.insert((step, index), res);
        res
    }
}

impl Solution {
    pub fn num_ways(steps: i32, arr_len: i32) -> i32 {
        let mut dp = DP {
            steps,
            arr_len,
            cache: HashMap::new(),
        };

        dp.dp(1, 0);
        dp.cache[&(1, 0)] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::num_ways(1, 1), 1);

        assert_eq!(Solution::num_ways(3, 2), 4);
        assert_eq!(Solution::num_ways(2, 4), 2);
        assert_eq!(Solution::num_ways(4, 2), 8);

        assert_eq!(Solution::num_ways(27, 7), 127784505);
    }
}