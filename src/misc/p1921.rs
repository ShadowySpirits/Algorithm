//! 1921. Eliminate Maximum Number of Monsters

pub struct Solution;

impl Solution {
    pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
        let count = dist.len() as i32;

        let mut monster_vec = vec![];
        for (i, dist) in dist.into_iter().enumerate() {
            let carry = if dist % speed[i] == 0 { 0 } else { 1 };
            monster_vec.push(dist / speed[i] + carry)
        }
        monster_vec.sort();

        for (i, time) in monster_vec.into_iter().enumerate() {
            if time <= i as i32 {
                return i as i32;
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::eliminate_maximum(vec![1, 3, 4], vec![1, 1, 1]), 3);
        assert_eq!(Solution::eliminate_maximum(vec![1, 1, 2, 3], vec![1, 1, 1, 1]), 1);
        assert_eq!(Solution::eliminate_maximum(vec![3, 2, 4], vec![5, 3, 2]), 1);
        assert_eq!(Solution::eliminate_maximum(vec![4, 2, 8], vec![2, 1, 4]), 2);
        assert_eq!(Solution::eliminate_maximum(vec![4, 2, 8, 3, 9], vec![2, 1, 4, 5, 3]), 2);
    }
}