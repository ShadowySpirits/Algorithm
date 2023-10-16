//! 119. Pascal's Triangle II

pub struct Solution;

impl Solution {
    // Like p118, but only return the row_index-th row.
    // So that we can only cache last row.
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut last_vec = Vec::with_capacity(row_index as usize + 1);

        for row in 0..=row_index as usize {
            let mut cur_vec = vec![1; row + 1];

            if cur_vec.len() > 2 {
                for i in 1..row {
                    cur_vec[i] = last_vec[i - 1] + last_vec[i];
                }
            }

            last_vec = cur_vec;
        }
        last_vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let row_index = 3;
        let expected = vec![1, 3, 3, 1];
        assert_eq!(Solution::get_row(row_index), expected);

        let row_index = 0;
        let expected = vec![1];
        assert_eq!(Solution::get_row(row_index), expected);

        let row_index = 1;
        let expected = vec![1, 1];
        assert_eq!(Solution::get_row(row_index), expected);
    }
}