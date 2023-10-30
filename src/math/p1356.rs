//! 1356. Sort Integers by The Number of 1 Bits

use std::cmp::Ordering;

pub struct Solution;

#[derive(Debug, PartialEq, Eq)]
struct U32Wrapper {
    inner: u32
}


impl PartialOrd for U32Wrapper {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_weight = crate::math::p191::Solution::hamming_weight(self.inner);
        let other_weight = crate::math::p191::Solution::hamming_weight(other.inner);

        match self_weight.cmp(&other_weight) {
            Ordering::Less => Some(Ordering::Less),
            Ordering::Equal => self.inner.partial_cmp(&other.inner),
            Ordering::Greater => Some(Ordering::Greater)
        }
    }
}

impl Ord for U32Wrapper {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Solution {
    pub fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
        let mut vec: Vec<U32Wrapper> = arr.into_iter().map(|inner| U32Wrapper { inner: inner as u32 }).collect();
        vec.sort();
        vec.into_iter().map(|wrapper| wrapper.inner as i32).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::sort_by_bits(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]), vec![0, 1, 2, 4, 8, 3, 5, 6, 7]);
        assert_eq!(Solution::sort_by_bits(vec![1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1]), vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]);
        assert_eq!(Solution::sort_by_bits(vec![10000, 10000]), vec![10000, 10000]);
        assert_eq!(Solution::sort_by_bits(vec![2, 3, 5, 7, 11, 13, 17, 19]), vec![2, 3, 5, 17, 7, 11, 13, 19]);
        assert_eq!(Solution::sort_by_bits(vec![10, 100, 1000, 10000]), vec![10, 100, 10000, 1000]);
    }
}