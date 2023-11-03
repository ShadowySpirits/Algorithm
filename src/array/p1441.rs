//! 1441. Build an Array With Stack Operations

pub struct Solution;

impl Solution {
    pub fn build_array(target: Vec<i32>, _n: i32) -> Vec<String> {
        let mut res = vec![];
        let mut i = 1;

        for num in target {
            while i < num {
                res.push("Push".to_string());
                res.push("Pop".to_string());
                i += 1;
            }

            res.push("Push".to_string());
            i += 1;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::build_array(vec![1, 3], 3), vec!["Push", "Push", "Pop", "Push"]);
        assert_eq!(Solution::build_array(vec![1, 2, 3], 3), vec!["Push", "Push", "Push"]);
        assert_eq!(Solution::build_array(vec![1, 2], 4), vec!["Push", "Push"]);
        assert_eq!(Solution::build_array(vec![2, 3, 4], 4), vec!["Push", "Pop", "Push", "Push", "Push"]);
    }
}