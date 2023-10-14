//! 1095. Find in Mountain Array

pub struct Solution;

pub struct MountainArray;

impl MountainArray {
    fn get(&self, _index: i32) -> i32 {
        unimplemented!()
    }

    fn length(&self) -> i32 {
        unimplemented!()
    }
}

impl Solution {
    fn find_peak_index(arr: &MountainArray) -> i32 {
        let mut left = 0;
        let mut right = arr.length() - 1;
        let mut mid = (left + right) / 2;

        while left < right {
            if arr.get(mid) < arr.get(mid + 1) {
                left = mid + 1;
            } else {
                right = mid;
            }
            mid = (left + right) / 2;
        }

        mid
    }

    fn binary_search(target: i32, arr: &MountainArray, left: i32, right: i32) -> Option<i32> {
        let mut left = left;
        let mut right = right;
        let mut mid = (left + right) / 2;

        while left < right {
            match arr.get(mid).cmp(&target) {
                std::cmp::Ordering::Less => left = mid + 1,
                std::cmp::Ordering::Equal => return Some(mid),
                std::cmp::Ordering::Greater => right = mid,
            }
            mid = (left + right) / 2
        }

        None
    }

    pub fn find_in_mountain_array(target: i32, mountain_arr: &MountainArray) -> i32 {
        // Step1. Find the peak, see p852 for details.
        let peak_index = Self::find_peak_index(mountain_arr);
        // Step2. Binary search in the left part.
        let left = Self::binary_search(target, mountain_arr, 0, peak_index);
        // Step3. If find nothing, then binary search in the right part.
        if left.is_none() {
            return Self::binary_search(target, mountain_arr, peak_index + 1, mountain_arr.length() - 1).unwrap();
        }

        left.unwrap()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}