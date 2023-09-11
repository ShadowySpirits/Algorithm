//! 1282. Group the People Given the Group Size They Belong To

use std::collections::BTreeMap;

pub struct Solution;

impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        // Map<group_size, people_vec>
        // Because order does not matter, HashMap is okay, but using TreeMap this time just to let the test pass.
        let mut map = BTreeMap::new();
        for (i, size) in group_sizes.iter().enumerate() {
            map.entry(*size)
                .or_insert_with(Vec::new)
                .push(i as i32);
        }

        let mut res = vec![];
        for (size, people_with_same_size) in map {
            // split into multiple vec with required size
            if people_with_same_size.len() > size as usize {
                let mut i = 0;
                loop {
                    if i >= people_with_same_size.len() {
                        break;
                    }
                    res.push(people_with_same_size[i..i + size as usize].to_vec());
                    i += size as usize;
                }
            } else {
                res.push(people_with_same_size)
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
        // Notice: order does not not matter
        assert_eq!(
            Solution::group_the_people(vec![3, 3, 3, 3, 3, 1, 3]),
            vec![vec![5], vec![0, 1, 2], vec![3, 4, 6]]
        );
        assert_eq!(
            Solution::group_the_people(vec![2, 1, 3, 3, 3, 2]),
            vec![vec![1], vec![0, 5], vec![2, 3, 4]]
        );
    }
}