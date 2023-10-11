//! 2251. Number of Flowers in Full Bloom

use std::cmp::Ordering;
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    // Copy from algo::p2009
    fn binary_search(nums: &Vec<i32>, upper_bound: i32) -> Option<usize> {
        let (mut left, mut right) = (0, nums.len());
        let mut mid = (left + right) / 2;

        while left < right {
            match nums[mid].cmp(&upper_bound) {
                Ordering::Less => left = mid + 1,
                Ordering::Equal => return Some(Self::find_right_bound(nums, upper_bound, mid)),
                Ordering::Greater => right = mid,
            }
            mid = (left + right) / 2;
        }

        if mid == 0 {
            None
        } else {
            Some(Self::find_right_bound(nums, upper_bound, mid - 1))
        }
    }

    fn find_right_bound(nums: &Vec<i32>, target: i32, index: usize) -> usize {
        let mut right_bound = index;
        while right_bound < nums.len() - 1 && nums[right_bound + 1] == target {
            right_bound += 1;
        }
        right_bound
    }

    pub fn full_bloom_flowers(flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32> {
        let mut start_vec = Vec::new();
        let mut end_vec = Vec::new();

        for range in flowers.into_iter() {
            let start = range[0];
            let end = range[1];
            start_vec.push(start);
            end_vec.push(end);
        }

        start_vec.sort();
        end_vec.sort();

        let mut res = Vec::with_capacity(people.len());
        for time in people {
            // search the flowers that bloom before people arrive.
            match Self::binary_search(&start_vec, time) {
                None => res.push(0),
                // search the flowers that stop blooming before people arrive.
                Some(start_index) => {
                    match Self::binary_search(&end_vec, time - 1) {
                        // If no flower stop blooming before people arrive.
                        // bloom_count = start_index - 0 + 1
                        // stop_bloom_count = 0
                        // still_bloom_count = bloom_count - stop_bloom_count
                        None => res.push(start_index as i32 + 1),
                        // If some flowers end blooming before people arrive, we need to filter them out.
                        // Notice: the flowers that stop blooming before people arrive must also bloom before people arrive.
                        // bloom_count = start_index - 0 + 1
                        // stop_bloom_count = end_index - 0 + 1
                        // still_bloom_count = bloom_count - stop_bloom_count
                        Some(end_index) => res.push((start_index - end_index) as i32)
                    }
                }
            }
        }

        res
    }

    // Build bloom map lead to TLE
    pub fn hash_map_solution(flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32> {
        let mut bloom_map = HashMap::new();
        for range in flowers {
            let start = range[0];
            let end = range[1];
            for time in start..=end {
                let flower_count = bloom_map.get(&time).unwrap_or(&0);
                bloom_map.insert(time, flower_count + 1);
            }
        }

        let mut res = Vec::new();
        for time in people {
            let flower_count = bloom_map.get(&time).unwrap_or(&0);
            res.push(*flower_count);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let flowers = vec![vec![1, 6], vec![3, 7], vec![9, 12], vec![4, 13]];
        let people = vec![2, 3, 7, 11];
        let ans = vec![1, 2, 2, 2];
        assert_eq!(Solution::full_bloom_flowers(flowers, people), ans);

        let flowers = vec![vec![1, 10], vec![3, 3]];
        let people = vec![3, 3, 2];
        let ans = vec![2, 2, 1];
        assert_eq!(Solution::full_bloom_flowers(flowers, people), ans);

        let flowers = vec![vec![28, 37], vec![23, 33], vec![39, 39], vec![49, 50], vec![41, 45], vec![14, 47]];
        let people = vec![48];
        let ans = vec![0];
        assert_eq!(Solution::full_bloom_flowers(flowers, people), ans);
    }
}