//! 2849. Determine if a Cell Is Reachable at a Given Time

use std::cmp::max;

pub struct Solution;

impl Solution {
    pub fn is_reachable_at_time(sx: i32, sy: i32, fx: i32, fy: i32, t: i32) -> bool {
        let delta_x = (fx - sx).abs();
        let delta_y = (fy - sy).abs();

        if delta_x == 0 && delta_y == 0 && t == 1 {
            return false;
        }

        t >= max(delta_x, delta_y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::is_reachable_at_time(2, 4, 7, 7, 6), true);
        assert_eq!(Solution::is_reachable_at_time(3, 1, 7, 3, 3), false);
        assert_eq!(Solution::is_reachable_at_time(1, 1, 3, 5, 2), false);
        assert_eq!(Solution::is_reachable_at_time(1, 1, 1, 1, 1), false);
        assert_eq!(Solution::is_reachable_at_time(1, 1, 1, 1, 2), true);
        assert_eq!(Solution::is_reachable_at_time(1, 1, 1, 1, 3), true);
        assert_eq!(Solution::is_reachable_at_time(1, 1, 3, 2, 3), true);
        assert_eq!(Solution::is_reachable_at_time(1, 1, 2, 1, 2), true);
    }
}