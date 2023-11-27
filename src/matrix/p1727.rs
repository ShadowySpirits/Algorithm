//! 1727. Largest Submatrix With Rearrangements

use std::cmp::Reverse;

pub struct Solution;

impl Solution {
    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        let mut row_steak = vec![0; matrix[0].len()];

        let mut res = 0;
        for col in matrix {
            for (index, num) in col.into_iter().enumerate() {
                if num == 1 {
                    row_steak[index] += 1;
                } else {
                    row_steak[index] = 0;
                }
            }

            let mut copy = row_steak.iter().map(|num| Reverse(*num)).collect::<Vec<Reverse<i32>>>();
            copy.sort_unstable();
            for (index, Reverse(num)) in copy.into_iter().enumerate() {
                let steak = index as i32 + 1;
                res = res.max(num * steak);
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
        assert_eq!(Solution::largest_submatrix(vec![vec![0, 0, 1], vec![1, 1, 1], vec![1, 0, 1]]), 4);
        assert_eq!(Solution::largest_submatrix(vec![vec![1, 0, 1, 0, 1]]), 3);
        assert_eq!(Solution::largest_submatrix(vec![vec![1, 1, 0], vec![1, 0, 1]]), 2);
        assert_eq!(Solution::largest_submatrix(vec![vec![0, 0], vec![0, 0]]), 0);
    }
}