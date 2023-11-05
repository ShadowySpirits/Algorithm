//! 1535. Find the Winner of an Array Game

pub struct Solution;

impl Solution {
    pub fn get_winner(arr: Vec<i32>, k: i32) -> i32 {

        let mut max = arr[0];
        let mut win_times = 0;

        // If some number lose the game, it will never be the winner. So we just drop it.
        // And we can stop the loop when we exhaust all numbers, which means we find the max number.
        for next in arr.into_iter().skip(1) {
            if max < next {
                max = next;
                win_times = 0;
            }
            win_times += 1;

            // If we find the winner, we can break the loop.
            if win_times == k {
                break;
            }
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::get_winner(vec![2, 1, 3, 5, 4, 6, 7], 2), 5);
        assert_eq!(Solution::get_winner(vec![3, 2, 1], 10), 3);
        assert_eq!(Solution::get_winner(vec![1, 9, 8, 2, 3, 7, 6, 4, 5], 7), 9);
        assert_eq!(Solution::get_winner(vec![1, 11, 22, 33, 44, 55, 66, 77, 88, 99], 1000000000), 99);
    }
}