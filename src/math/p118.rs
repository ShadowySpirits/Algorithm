//! 118. Pascal's Triangle

use std::iter;

struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::with_capacity(num_rows as usize);
        for row in 0..num_rows as usize {
            let mut cur_vec: Vec<i32> = iter::repeat(1).take(row + 1).collect();

            if cur_vec.len() > 2 {
                let last_vec: &Vec<i32> = &res[row - 1];
                for i in 1..row {
                    cur_vec[i] = last_vec[i - 1] + last_vec[i];
                }
            }

            res.push(cur_vec);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let num_rows = 5;
        let expected = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
        ];
        assert_eq!(Solution::generate(num_rows), expected);
    }
}