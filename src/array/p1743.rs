//! 1743. Restore the Array From Adjacent Pairs

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
        let mut map = HashMap::new();
        for pair in adjacent_pairs {
            map.entry(pair[0]).or_insert(vec![]).push(pair[1]);
            map.entry(pair[1]).or_insert(vec![]).push(pair[0]);
        }

        let mut last = i32::MIN;
        let mut current = 0;
        for (num, set) in &map {
            if set.len() == 1 {
                current = *num;
                break;
            }
        }

        let mut res = vec![];

        loop {
            let vec = map.remove(&current).unwrap();
            
            let mut next = i32::MIN;
            for num in vec {
                if num != last {
                    next = num;
                    break;
                }
            }

            res.push(current);

            if next == i32::MIN {
                break;
            }

            last = current;
            current = next;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::restore_array(vec![vec![2, 1], vec![3, 4], vec![3, 2]]),
            vec![1, 2, 3, 4]
        );
        assert_eq!(
            Solution::restore_array(vec![vec![4, -2], vec![1, 4], vec![-3, 1]]),
            vec![-2, 4, 1, -3]
        );
        assert_eq!(
            Solution::restore_array(vec![vec![100000, -100000]]),
            vec![100000, -100000]
        );
    }
}