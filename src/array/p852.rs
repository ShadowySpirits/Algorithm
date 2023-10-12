//! 852. Peak Index in a Mountain Array

pub struct Solution;

impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = arr.len() - 1;
        let mut mid = (left + right) / 2;

        while left < right {
            if arr[mid] < arr[mid + 1] {
                left = mid + 1;
            } else {
                right = mid;
            }
            mid = (left + right) / 2;
        }

        mid as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 1, 0]), 1);
        assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 2, 1, 0]), 1);
        assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 10, 5, 2]), 1);
        assert_eq!(Solution::peak_index_in_mountain_array(vec![3, 4, 5, 1]), 2);
        assert_eq!(Solution::peak_index_in_mountain_array(vec![24, 69, 100, 99, 79, 78, 67, 36, 26, 19]), 2);
    }
}