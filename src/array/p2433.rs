//! 2433. Find The Original Array of Prefix Xor

pub struct Solution;

impl Solution {
    pub fn find_array(pref: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::with_capacity(pref.len());
        res.push(pref[0]);

        for (i, value) in pref.iter().enumerate().skip(1) {
            res.push(value ^ pref[i - 1]);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::find_array(vec![5, 2, 0, 3, 1]), vec![5, 7, 2, 3, 2]);
        assert_eq!(Solution::find_array(vec![13]), vec![13]);
    }
}